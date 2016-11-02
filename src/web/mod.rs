extern crate clap;
extern crate hyper;
extern crate iron;
extern crate router;


mod handler;

use hyper::header::{ContentType, Headers, ETag, EntityTag};
use hyper::mime::{Attr, Mime, TopLevel, SubLevel, Value};

use iron::prelude::*;
use iron::status;
use router::Router;

use std::io::Read;
use std::sync::{Arc, Mutex};

use super::client::Client;
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

pub fn handler_get(safe_client: &Arc<Mutex<Client>>, req: &mut Request) -> IronResult<Response> {
    let ref docid = req.extensions.get::<Router>().unwrap().find("docid").unwrap_or("");
    let mut client = safe_client.lock().unwrap();

    let response = client.get_sync(docid);
    match response {
        Ok(result) => {
            let cas = result.cas.to_string();
            let value = result.value.unwrap();

            let mut headers = Headers::new();
            headers.set(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![(Attr::Charset, Value::Utf8)])));
            headers.set(ETag(EntityTag::new(false, cas.to_owned())));

            let mut response = Response::with((status::Ok, format!("{}\n", value)));
            response.headers = headers;
            Ok(response)
        },
        Err(res) => {
            Ok(
                Response::with((status::BadRequest, response::format_error(
                    *client.instance.as_ref().unwrap().lock().unwrap(),
                    &res.0.unwrap().rc ))
                )
            )
        }
    }
}

pub fn handler_store(safe_client: &Arc<Mutex<Client>>, operation: Operation, req: &mut Request) -> IronResult<Response> {
    let ref docid = req.extensions.get::<Router>().unwrap().find("docid").unwrap_or("");
    let mut client = safe_client.lock().unwrap();
    // let mut client = c.lock().unwrap();

    let mut payload = String::new();
    let _ = req.body.read_to_string(&mut payload).unwrap();
    let response = client.store_sync(docid, &payload[..], operation);
    match response {
        Ok(result) => {
            let cas = result.cas.to_string();

            let mut headers = Headers::new();
            headers.set(ContentType::plaintext());
            headers.set(ETag(EntityTag::new(false, cas.to_owned())));

            let mut response = Response::with((status::Ok, format!("{}\n", cas)));
            response.headers = headers;
            Ok(response)
        },
        Err(res) => {
            Ok(
                Response::with((status::BadRequest, response::format_error(
                    *client.instance.as_ref().unwrap().lock().unwrap(),
                    &res.0.unwrap().rc ))
                )
            )
        }
    }
}

#[allow(unused_mut)]
#[allow(unused_must_use)]
#[allow(unused_variables)]
pub fn start_web<'a>(c: &'a Arc<Mutex<Client>>, port: u16) {
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

    router.get("/bucket/:bucketid/doc/:docid", get_handler, "doc_get");

//    See https://github.com/ApolloCrawler/gauc/issues/4
//    router.delete("/bucket/:bucketid/doc/:docid", doc::delete::delete_handler, "doc_delete");

    router.post("/bucket/:bucketid/doc/:docid", create_handler, "doc_insert");
    router.post("/bucket/:bucketid/doc/:docid/add", add_handler, "doc_add");
    router.post("/bucket/:bucketid/doc/:docid/append", append_handler, "doc_append");
    router.post("/bucket/:bucketid/doc/:docid/prepend", prepend_handler, "doc_prepend");
    router.post("/bucket/:bucketid/doc/:docid/replace", replace_handler, "doc_replace");
    router.post("/bucket/:bucketid/doc/:docid/set", set_handler , "doc_set");
    router.post("/bucket/:bucketid/doc/:docid/upsert", upsert_handler, "doc_upsert");

    let address = format!("localhost:{}", port);
    match Iron::new(router).http(&address[..]) {
        Ok(_res) => {
            // println!("{:?}", res);
        },
        Err(res) => {
            println!("{:?}", res);
        }
    }
}
