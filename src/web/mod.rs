extern crate clap;
extern crate iron;
extern crate router;

mod handler;

use iron::prelude::*;
use iron::status;
use router::Router;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

// use std::ptr::Unique;
use super::client::Client;

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

#[derive(Debug)]
pub struct IronResponse {
    pub data: String
}

#[derive(Debug)]
pub struct IronRequest {
    pub url: iron::Url,
    pub method: iron::method::Method,
    pub tx: Arc<Mutex<Sender<IronResponse>>>
}

#[allow(unused_mut)]
#[allow(unused_must_use)]
#[allow(unused_variables)]
pub fn start_web(port: u16, tx: Arc<Mutex<Sender<IronRequest>>>) {
    println!("Starting REST Interface on port {}.", port);

    let mut router = Router::new();

    let c = Client::new("couchbase://localhost/default");

    let get_handler = move |req: &mut Request| -> IronResult<Response> {
        debug!("{:?}", req);

        // let _ = c.get_sync("foo");

        let (client_tx, client_rx): (Sender<IronResponse>, Receiver<IronResponse>) = mpsc::channel();
        let shared_client_tx = Arc::new(Mutex::new(client_tx));

        let msg = IronRequest {
            url: req.url.clone(),
            method: req.method.clone(),
            tx: shared_client_tx.clone()
        };

        tx.lock().unwrap().send(msg);
        let res = client_rx.recv();
        Ok(Response::with((status::Ok, &res.unwrap().data[..])))
    };

    router.get("/bucket/:bucketid/doc/:docid", get_handler, "doc_get");
//    router.get("/bucket/:bucketid/doc/:docid", doc::get::get_handler, "doc_get");
//    router.delete("/bucket/:bucketid/doc/:docid", doc::delete::delete_handler, "doc_delete");
//    router.post("/bucket/:bucketid/doc/:docid", doc::upsert::upsert_handler, "doc_insert");
//    router.post("/bucket/:bucketid/doc/:docid/add", doc::add::add_handler, "doc_add");
//    router.post("/bucket/:bucketid/doc/:docid/append", doc::append::append_handler, "doc_append");
//    router.post("/bucket/:bucketid/doc/:docid/prepend", doc::prepend::prepend_handler, "doc_prepend");
//    router.post("/bucket/:bucketid/doc/:docid/replace", doc::replace::replace_handler, "doc_replace");
//    router.post("/bucket/:bucketid/doc/:docid/set", doc::set::set_handler, "doc_set");
//    router.post("/bucket/:bucketid/doc/:docid/upsert", doc::upsert::upsert_handler, "doc_upsert");

    let address = format!("localhost:{}", port);
    Iron::new(router).http(&address[..]).unwrap();
}
