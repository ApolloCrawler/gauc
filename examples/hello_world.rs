extern crate gauc;
extern crate env_logger;

use gauc::client::*;
use std::{thread, time};

fn main() {
    env_logger::init().unwrap();

    let mut client = Client::new("couchbase://korczis.com/default");

    for i in 0..100 {
        println!("Iteration #{}", i);

        // Store some data
        client.upsert(&format!("foo {}", i), &format!("This is test No. {}!", i), |res| {
            if let Ok(response) = res {
                println!("Created new document, CAS: {}", response.cas)
            }
        });
    }
    for i in 0..100 {
        // Get data
        client.get(&format!("foo {}", i), |res| {
            if let Ok(response) = res {
                println!("{} - {}", response.key.unwrap(), response.value.unwrap())
            }
        });
    }

    let sleep_interval = time::Duration::from_millis(1000);
    thread::sleep(sleep_interval);
}
