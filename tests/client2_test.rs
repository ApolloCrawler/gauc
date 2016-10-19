extern crate gauc;

use gauc::client::Client2;
use gauc::client::CouchbaseClient;

const DEFAULT_CONNECTION_STRING: &'static str = "couchbase://localhost/default";

#[test]
fn client2_constructor() {
    let _client = Client2::new();
}

#[test]
fn client2_get() {
    assert!(true);
}

#[test]
fn client2_parallel() {
    let client = Client2::new();

    const ITERATIONS: i32 = 64;
    for i in 0..ITERATIONS{
        let _ = client.get("foo");
    };
}

