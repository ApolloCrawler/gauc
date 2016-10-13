extern crate gauc;

use gauc::client::*;
use gauc::couchbase::types::response_get::ResponseGet;
use gauc::couchbase::types::response_store::ResponseStore;

fn main() {
    let mut client = Client::new("couchbase://localhost/default");

    fn cb_set(response: &ResponseStore) {
        println!("Data stored.");
        println!("{:?   }", response);
    };

    // Store some data
    client.store("bar", "{\"msg\": \"This is test!\"}", &cb_set);

    fn cb_get(response: &ResponseGet) {
        println!("{}", response.value());
    };

    // Get data - use function
    client.get("foo", &cb_get);

    // Get data - use in-place closure
    client.get("foo", |response: &ResponseGet| println!("{}", response.value()));
}
