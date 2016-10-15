extern crate gauc;

use gauc::client::*;

fn main() {
    let mut client = Client::new("couchbase://localhost/default");

    // Store some data
    client.store("foo", "{\"msg\": \"This is test!\"}", |response| {
        println!("Created new document, CAS: {}", response.cas)
    });

    // Get data
    client.get("foo", |response| {
        println!("{} - {}", response.key(), response.value())
    });
}
