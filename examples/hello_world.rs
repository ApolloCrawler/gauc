extern crate gauc;
extern crate env_logger;

use gauc::client::*;

fn main() {
    env_logger::init().unwrap();

    const NUM_ITERATIONS: i32 = 100;

    let mut client = Client::new();
    client.connect("couchbase://korczis.com/default");

    for i in 0..NUM_ITERATIONS {
        println!("Iteration #{}", i);

        // Store some data
        client.upsert(&format!("foo{}", i), &format!("{{\"msg\": \"This is test No. {}!\"}}", i), |res| {
            if let Ok(response) = res {
                println!("Created new document, CAS: {}", response.cas)
            }
        });

        // Get data
        client.get(&format!("foo{}", i), |res| {
            if let Ok(response) = res {
                println!("Got response: {} - {}", response.key.unwrap(), response.value.unwrap())
            }
        });
    }
}
