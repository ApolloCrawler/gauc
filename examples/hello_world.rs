extern crate gauc;
extern crate env_logger;

use gauc::client::*;
use std::{thread, time};

fn main() {
    env_logger::init().unwrap();

    let mut client = Client::new("couchbase://localhost/default");

//     println!("{:?}", client.get_sync("foo"));

    for i in 0..2 {
        println!("Iteration #{}", i);

        // Store some data
        client.upsert("foo", "{\"msg\": \"This is test!\"}", |res| {
            if let Ok(response) = res {
                // println!("Created new document, CAS: {}", response.cas)
            }
        });

//        let sleep_interval = time::Duration::from_millis(100);
//        thread::sleep(sleep_interval);

        // Get data
        client.get("foo", |res| {
            if let Ok(response) = res {
                // println!("{} - {}", response.key.unwrap(), response.value.unwrap())
            }
        });
    }

    let sleep_interval = time::Duration::from_millis(1000);
    thread::sleep(sleep_interval);
}
