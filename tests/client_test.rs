extern crate gauc;

use gauc::client::*;
//use gauc::couchbase::types::error_type::ErrorType;
//use gauc::couchbase::types::operation::Operation;

const DEFAULT_CONNECTION_STRING: &'static str = "couchbase://localhost/default";

#[test]
fn it_connects() {
    if let Ok(client) = Client::connect(DEFAULT_CONNECTION_STRING) {
        assert_eq!(client.opts.version(), 3);
    }
}

//#[test]
//fn it_stores_document() {
//    if let Ok(mut client) = Client::connect(DEFAULT_CONNECTION_STRING) {
//        // Store some data
//        client.store("foo", "{\"msg\": \"This is test!\"}", Operation::Upsert, 0, 0, |res| {
//            if let Ok(response) = res {
//                assert_eq!(response.rc, ErrorType::Success);
//                println!("Created new document, CAS: {}", response.cas)
//            }
//        });
//    }
//}
