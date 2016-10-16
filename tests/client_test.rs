extern crate gauc;

use gauc::client::*;
use gauc::couchbase::types::error_type::ErrorType;

const DEFAULT_CONNECTION_STRING: &'static str = "couchbase://localhost/default";

#[test]
fn it_connects() {
    let client = Client::new(DEFAULT_CONNECTION_STRING);
    assert!(client.opts.version() == 3);
}

#[test]
fn it_stores_document() {
    let mut client = Client::new(DEFAULT_CONNECTION_STRING);

    // Store some data
    client.store("foo", "{\"msg\": \"This is test!\"}", |res| {
        if let Ok(response) = res {
            assert!(response.rc == ErrorType::Success);
            println!("Created new document, CAS: {}", response.cas)
        }
    });
}
