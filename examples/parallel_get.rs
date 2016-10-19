extern crate env_logger;
extern crate gauc;

use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::{time};

use gauc::client::Client2;
use gauc::client::Request;
use gauc::client::Proxy;
use gauc::client::Worker;

const DEFAULT_CONNECTION_STRING: &'static str = "couchbase://localhost/default";

fn main() {
    env_logger::init().unwrap();

    // Create channel for communication with worker
    let (tx, rx): (Sender<Request>, Receiver<Request>) = mpsc::channel();
    let safe_rx = Arc::new(Mutex::new(rx));
    let safe_tx = Arc::new(Mutex::new(tx));

    // Create worker
    let _worker = Arc::new(Mutex::new(Worker::new(safe_rx, DEFAULT_CONNECTION_STRING)));

    // Create proxy
    let proxy  = Arc::new(Mutex::new(Proxy::new(safe_tx.clone())));

    // Create client
    let _client = Client2::new();

    const ITERATIONS: i32 = 2;
    const DEFAULT_KEY: &'static str = "foo";

    // Normal loop
    for _id in 0..ITERATIONS {
        let res = proxy.lock().unwrap().get(DEFAULT_KEY);
        println!("{:?}", res);
    }

    // Threaded loop
    let threads = (0..ITERATIONS).map(|_id| {
        let cloned_proxy = proxy.clone();
        thread::spawn(move || {
            cloned_proxy.lock().unwrap().get(DEFAULT_KEY);
        })
    });

    // Wait for child threads
    for child in threads {
        // Wait for the thread to finish. Returns a result.
        let res = child.join();
        println!("{:?}", res);
    }

    // Request quit
    proxy.lock().unwrap().quit();

    // Wait for a while before finish
    let sleep_interval = time::Duration::from_millis(500);
    thread::sleep(sleep_interval);
}
