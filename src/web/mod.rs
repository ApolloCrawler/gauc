extern crate clap;
extern crate hyper;
extern crate iron;
extern crate router;
extern crate serde;
extern crate serde_json;
extern crate urlencoded;

use hyper::header::{ContentType, Headers, ETag, EntityTag};
use hyper::mime::{Attr, Mime, TopLevel, SubLevel, Value};

use iron::prelude::*;
use iron::status;
use router::Router;
use serde_json::Map;

use std::io::Read;
use std::sync::{Arc, Mutex};
use urlencoded::UrlEncodedQuery;

use super::client::Client;
use super::couchbase::types::error_type;
use super::couchbase::types::instance;
use super::couchbase::types::response;
use super::couchbase::types::operation::Operation;

// Bucket REST Interface
//
// GET  /bucket/<BUCKET_NAME>/doc - list
// POST /bucket/<BUCKET_NAME>/flush - flush
//
// DELETE  /bucket/<BUCKET_NAME>/doc/<ID>            - delete *
// GET     /bucket/<BUCKET_NAME>/doc/<ID>            - get *
// POST    /bucket/<BUCKET_NAME>/doc/<ID>            - upsert *
// POST    /bucket/<BUCKET_NAME>/doc/<ID>/add        - add *
// POST    /bucket/<BUCKET_NAME>/doc/<ID>/append     - append *
// POST    /bucket/<BUCKET_NAME>/doc/<ID>/prepend    - prepend *
// POST    /bucket/<BUCKET_NAME>/doc/<ID>/replace    - replace *
// POST    /bucket/<BUCKET_NAME>/doc/<ID>/set        - set *
// POST    /bucket/<BUCKET_NAME>/doc/<ID>/upsert     - upsert (explitcit) *

pub fn get_meta(cas: &str, version: u16) -> Map<String, serde_json::Value> {
    let mut res: Map<String, serde_json::Value> = Map::new();

    res.insert("cas".to_string(), serde_json::value::Value::String(cas.to_string()));
    res.insert("version".to_string(), serde_json::value::Value::U64(u64::from(version)));

    res
}

pub fn get_error(client: instance::InstancePtr, rc: &error_type::ErrorType) -> Map<String, serde_json::Value> {
    let error = response::format_error(client, rc);

    let mut res: Map<String, serde_json::Value> = Map::new();
    res.insert("error".to_string(), serde_json::value::Value::String(error.to_string()));

    res
}

pub fn handler_get(safe_client: &Arc<Mutex<Client>>, req: &mut Request) -> IronResult<Response> {
    let mut cas: u64 = 0;

    if let Ok(hashmap) = req.get_ref::<UrlEncodedQuery>() {
        if hashmap.contains_key("cas") {
            let tmp = hashmap.get("cas").unwrap().last().unwrap();
            if let Ok(val) = tmp.parse::<u64>() {
                cas = val;
            }
        }
    };

    let docid = req.extensions.get::<Router>().unwrap().find("docid").unwrap_or("");
    let mut client = safe_client.lock().unwrap();

    let response = client.get_sync(docid, cas);
    match response {
        Ok(result) => {
            let cas = result.cas.to_string();
            let value = result.value.unwrap();

            let mut headers = Headers::new();
            headers.set(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![(Attr::Charset, Value::Utf8)])));
            headers.set(ETag(EntityTag::new(false, cas.to_owned())));

            let mut map = Map::new();
            map.insert("meta".to_string(), get_meta(&cas, result.version));

            let doc: Map<String, serde_json::Value> = serde_json::from_str(&value).unwrap();
            map.insert("doc".to_string(), doc);


            // TODO: Handle non-json documents
            let json = serde_json::to_string(&map).unwrap();

            let mut response = Response::with((status::Ok, json));
            response.headers = headers;
            Ok(response)
        },
        Err(res) => {
            let mut headers = Headers::new();
            headers.set(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![(Attr::Charset, Value::Utf8)])));

            let json = serde_json::to_string(
                &get_error(
                    client.instance,
                    &res.0.unwrap().rc
                )
            ).unwrap();

            let mut response = Response::with((status::BadRequest, json));
            response.headers = headers;
            Ok(response)
        }
    }
}

pub fn handler_remove(safe_client: &Arc<Mutex<Client>>, req: &mut Request) -> IronResult<Response> {
    debug!("handler_remove() called");
    let docid = req.extensions.get::<Router>().unwrap().find("docid").unwrap_or("");
    let mut client = safe_client.lock().unwrap();

    let mut payload = String::new();
    let _ = req.body.read_to_string(&mut payload).unwrap();
    let response = client.remove_sync(docid);
    match response {
        Ok(result) => {
            let cas = result.cas.to_string();

            let mut headers = Headers::new();
            headers.set(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![(Attr::Charset, Value::Utf8)])));
            headers.set(ETag(EntityTag::new(false, cas.to_owned())));

            let mut map = Map::new();
            map.insert("meta".to_string(), get_meta(&cas, result.version));

            let json = serde_json::to_string(&map).unwrap();

            let mut response = Response::with((status::Ok, json));
            response.headers = headers;
            Ok(response)
        },
        Err(res) => {
            let mut headers = Headers::new();
            headers.set(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![(Attr::Charset, Value::Utf8)])));

            let json = serde_json::to_string(
                &get_error(
                    client.instance,
                    &res.0.unwrap().rc
                )
            ).unwrap();

            let mut response = Response::with((status::BadRequest, json));
            response.headers = headers;
            Ok(response)
        }
    }
}

pub fn handler_store(safe_client: &Arc<Mutex<Client>>, operation: Operation, req: &mut Request) -> IronResult<Response> {
    let mut cas: u64 = 0;
    let mut exptime: u32 = 0;

    if let Ok(hashmap) = req.get_ref::<UrlEncodedQuery>() {
        if hashmap.contains_key("cas") {
            let tmp = hashmap.get("cas").unwrap().last().unwrap();
            cas = match tmp.parse::<u64>() {
                Ok(val) => val,
                _ => 0
            };
        }

        if hashmap.contains_key("exptime") {
            let tmp = hashmap.get("exptime").unwrap().last().unwrap();
            exptime = match tmp.parse::<u32>() {
                Ok(val) => val,
                _ => 0
            };
        }
    };

    let docid = req.extensions.get::<Router>().unwrap().find("docid").unwrap_or("");
    let mut client = safe_client.lock().unwrap();

    let mut payload = String::new();
    let _ = req.body.read_to_string(&mut payload).unwrap();
    let response = client.store_sync(docid, &payload[..], operation, cas, exptime);

    match response {
        Ok(result) => {
            let cas = result.cas.to_string();

            let mut headers = Headers::new();
            headers.set(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![(Attr::Charset, Value::Utf8)])));
            headers.set(ETag(EntityTag::new(false, cas.to_owned())));

            let mut map = Map::new();
            map.insert("meta".to_string(), get_meta(&cas, result.version));

            let json = serde_json::to_string(&map).unwrap();

            let mut response = Response::with((status::Ok, json));
            response.headers = headers;
            Ok(response)
        },
        Err(res) => {
            let mut headers = Headers::new();
            headers.set(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![(Attr::Charset, Value::Utf8)])));

            let json = serde_json::to_string(
                &get_error(
                    client.instance,
                    &res.0.unwrap().rc
                )
            ).unwrap();

            let mut response = Response::with((status::BadRequest, json));
            response.headers = headers;
            Ok(response)
        }
    }
}

pub fn handler_view_query(safe_client: &Arc<Mutex<Client>>, req: &mut Request) -> IronResult<Response> {
    let ddoc = req.extensions.get::<Router>().unwrap().find("ddoc").unwrap_or("");
    let view = req.extensions.get::<Router>().unwrap().find("view").unwrap_or("");
    let mut client = safe_client.lock().unwrap();

    let response = client.view_query_sync(ddoc, view);
    match response {
        Err(res) => {
            let mut headers = Headers::new();
            headers.set(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![(Attr::Charset, Value::Utf8)])));

            let json = serde_json::to_string(
                &get_error(
                    client.instance,
                    &res.0.unwrap().rc
                )
            ).unwrap();

            let mut response = Response::with((status::BadRequest, json));
            response.headers = headers;
            Ok(response)
        },
        Ok(res) => {
            let cas = res.cas.to_string();
            let value = res.value.unwrap();

            let mut headers = Headers::new();
            headers.set(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![(Attr::Charset, Value::Utf8)])));
            headers.set(ETag(EntityTag::new(false, cas.to_owned())));

            let mut map = Map::new();
//            map.insert("meta".to_string(), get_meta(&cas, result.version));

            let doc: Map<String, serde_json::Value> = serde_json::from_str(&value).unwrap();
            map.insert("doc".to_string(), doc);

            // TODO: Handle non-json documents
            let json = serde_json::to_string(&map).unwrap();

            let mut response = Response::with((status::Ok, json));
            response.headers = headers;
            Ok(response)
        }
    }
}

#[allow(unused_mut)]
#[allow(unused_must_use)]
#[allow(unused_variables)]
pub fn start_web(c: &Arc<Mutex<Client>>, port: u16) {
    println!("Starting REST Interface on port {}.", port);

    let mut router = Router::new();

    // Add handler
    let handler_client = Arc::new(Mutex::new(c.lock().unwrap().clone()));
    let add_handler = move |req: &mut Request| -> IronResult<Response> {
        handler_store(&handler_client, Operation::Add, req)
    };

    // Append handler
    let handler_client = Arc::new(Mutex::new(c.lock().unwrap().clone()));
    let append_handler = move |req: &mut Request| -> IronResult<Response> {
        handler_store(&handler_client, Operation::Append, req)
    };

    // Create handler
    let handler_client = Arc::new(Mutex::new(c.lock().unwrap().clone()));
    let create_handler = move |req: &mut Request| -> IronResult<Response> {
        handler_store(&handler_client, Operation::Upsert, req)
    };

    // Get handler
    let handler_client = Arc::new(Mutex::new(c.lock().unwrap().clone()));
    let get_handler = move |req: &mut Request| -> IronResult<Response> {
        handler_get(&handler_client, req)
    };

    // Prepend handler
    let handler_client = Arc::new(Mutex::new(c.lock().unwrap().clone()));
    let prepend_handler = move |req: &mut Request| -> IronResult<Response> {
        handler_store(&handler_client, Operation::Prepend, req)
    };

    // Remove handler
    let handler_client = Arc::new(Mutex::new(c.lock().unwrap().clone()));
    let remove_handler = move |req: &mut Request| -> IronResult<Response> {
        handler_remove(&handler_client, req)
    };

    // Replace handler
    let handler_client = Arc::new(Mutex::new(c.lock().unwrap().clone()));
    let replace_handler = move |req: &mut Request| -> IronResult<Response> {
        handler_store(&handler_client, Operation::Replace, req)
    };

    // Set handler
    let handler_client = Arc::new(Mutex::new(c.lock().unwrap().clone()));
    let set_handler = move |req: &mut Request| -> IronResult<Response> {
        handler_store(&handler_client, Operation::Set, req)
    };

    // Upsert handler
    let handler_client = Arc::new(Mutex::new(c.lock().unwrap().clone()));
    let upsert_handler = move |req: &mut Request| -> IronResult<Response> {
        handler_store(&handler_client, Operation::Upsert, req)
    };

    // View query handler
    let handler_client = Arc::new(Mutex::new(c.lock().unwrap().clone()));
    let view_query_handler = move |req: &mut Request| -> IronResult<Response> {
        handler_view_query(&handler_client, req)
    };

    // Docs
    router.get("/bucket/:bucketid/doc/:docid", get_handler, "doc_get");
    router.post("/bucket/:bucketid/doc/:docid", create_handler, "doc_insert");
    router.post("/bucket/:bucketid/doc/:docid/add", add_handler, "doc_add");
    router.post("/bucket/:bucketid/doc/:docid/append", append_handler, "doc_append");
    router.post("/bucket/:bucketid/doc/:docid/prepend", prepend_handler, "doc_prepend");
    router.delete("/bucket/:bucketid/doc/:docid", remove_handler, "doc_remove");
    router.post("/bucket/:bucketid/doc/:docid/replace", replace_handler, "doc_replace");
    router.post("/bucket/:bucketid/doc/:docid/set", set_handler , "doc_set");
    router.post("/bucket/:bucketid/doc/:docid/upsert", upsert_handler, "doc_upsert");

    // Views
    router.get("/bucket/:bucketid/view/:ddoc/:view", view_query_handler, "view_query");

    let address = format!("0.0.0.0:{}", port);
    match Iron::new(router).http(&address[..]) {
        Ok(_res) => {
            // println!("{:?}", res);
        },
        Err(res) => {
            println!("{:?}", res);
        }
    }
}

