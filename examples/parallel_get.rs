extern crate env_logger;
extern crate gauc;

use std::sync::{Arc, Mutex};
use std::thread;

use gauc::client::Client2;
use gauc::client::CouchbaseClient;

const DEFAULT_CONNECTION_STRING: &'static str = "couchbase://localhost/default";

fn main() {
    env_logger::init().unwrap();

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
