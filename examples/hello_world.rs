extern crate gauc;

use gauc::client::*;

fn main() {
    let mut client = Client::new("couchbase://localhost/default");
    client
        .get("foo")
        .get("bar");

    client.wait_max(10000);
}
