extern crate gauc;

use gauc::client::*;
use gauc::couchbase::types::response_get::ResponseGet;
use gauc::couchbase::types::response_store::ResponseStore;

fn main() {
    let mut client = Client::new("couchbase://localhost/default");

    // Store some data
    client.store("foo", "{\"msg\": \"This is test!\"}", |response: &ResponseStore| {
        println!("Created new document, CAS: {}", response.cas)
    });

    // Get data
    client.get("foo", |response: &ResponseGet| {
        println!("{} - {}", response.key(), response.value())
    });
}
