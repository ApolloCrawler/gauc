extern crate gauc;

use std::sync::{Arc, Mutex};
// use std::sync::mpsc::{Sender, Receiver};
// use std::sync::mpsc;
use std::thread;

use gauc::client::Client2;
use gauc::client::CouchbaseClient;

const DEFAULT_CONNECTION_STRING: &'static str = "couchbase://localhost/default";

#[test]
fn client2_constructor() {
    let _client = Client2::new(DEFAULT_CONNECTION_STRING);
}

#[test]
fn client2_get() {
    assert!(true);
}

#[test]
fn client2_parallel() {
    let client = Arc::new(Mutex::new(Client2::new(DEFAULT_CONNECTION_STRING)));

    const NTHREADS: i32 = 64;
    let threads = (0..NTHREADS).map(|_| {
        let cloned_client = client.clone();
        thread::spawn(move || {
            cloned_client.lock().unwrap().get("foo");
        })
    });

    for child in threads {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}
