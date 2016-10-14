extern crate gauc;

use gauc::client::*;
use gauc::couchbase::types::response;

fn main() {
    let mut client = Client::new("couchbase://localhost/default");

    // Store some data
    client.store("foo", "{\"msg\": \"This is test!\"}", |response: &response::Store| {
        println!("Created new document, CAS: {}", response.cas)
    });

    // Get data
    client.get("foo", |response: &response::Get| {
        println!("{} - {}", response.key(), response.value())
    });
}
