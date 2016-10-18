extern crate gauc;

use gauc::client::*;

fn main() {
    let mut client = Client::new("couchbase://localhost/default");

    // Store some data
    client.upsert("foo", "{\"msg\": \"This is test!\"}", |res| {
        if let Ok(response) = res {
            println!("Created new document, CAS: {}", response.cas)
        }
    });

    // Get data
    client.get("foo", |res| {
        if let Ok(response) = res {
            println!("{} - {}", response.key.unwrap(), response.value.unwrap())
        }
    });
}
