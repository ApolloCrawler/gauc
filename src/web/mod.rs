extern crate clap;
extern crate iron;
extern crate router;

mod handler;

use iron::mime;
use iron::prelude::*;
use iron::status;
use router::Router;

use std::sync::{Arc, Mutex};

// use std::ptr::Unique;
use super::client::Client;
use super::couchbase::types::response;

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

#[allow(unused_mut)]
#[allow(unused_must_use)]
#[allow(unused_variables)]
pub fn start_web(port: u16) {
    println!("Starting REST Interface on port {}.", port);

    let mut router = Router::new();

    let mut c = Arc::new(Mutex::new(Client::new()));
    c.lock().unwrap().connect("couchbase://localhost/default");

    // Get handler
    let handler_client = Arc::new(Mutex::new(c.lock().unwrap().clone()));
    let get_handler = move |req: &mut Request| -> IronResult<Response> {
        let ref docid = req.extensions.get::<Router>().unwrap().find("docid").unwrap_or("");
        let mut client = handler_client.lock().unwrap();

        match client.get_sync(docid) {
            Ok(result) => {
                let content_type = mime::Mime(iron::mime::TopLevel::Application, iron::mime::SubLevel::Json, vec![]);
                Ok(Response::with((content_type, status::Ok, format!("{}\n", result.value.unwrap()))))
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
    };

    // Upsert handler
    let handler_client = Arc::new(Mutex::new(c.lock().unwrap().clone()));
    let replace_handler = move |req: &mut Request| -> IronResult<Response> {
        let res = c.lock().unwrap().clone().upsert_sync("abc", "def");
        Ok(Response::with((status::Ok, "")))
    };

    router.get("/bucket/:bucketid/doc/:docid", get_handler, "doc_get");

//    router.get("/bucket/:bucketid/doc/:docid", doc::get::get_handler, "doc_get");
//    router.delete("/bucket/:bucketid/doc/:docid", doc::delete::delete_handler, "doc_delete");
//    router.post("/bucket/:bucketid/doc/:docid", doc::upsert::upsert_handler, "doc_insert");
//    router.post("/bucket/:bucketid/doc/:docid/add", doc::add::add_handler, "doc_add");
//    router.post("/bucket/:bucketid/doc/:docid/append", doc::append::append_handler, "doc_append");
//    router.post("/bucket/:bucketid/doc/:docid/prepend", doc::prepend::prepend_handler, "doc_prepend");
    router.post("/bucket/:bucketid/doc/:docid/replace", replace_handler, "doc_replace");
//    router.post("/bucket/:bucketid/doc/:docid/set", set_handler , "doc_set");
//    router.post("/bucket/:bucketid/doc/:docid/upsert", upsert_handler, "doc_upsert");

    let address = format!("localhost:{}", port);
    Iron::new(router).http(&address[..]).unwrap();
}
