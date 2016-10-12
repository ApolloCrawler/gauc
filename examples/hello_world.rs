extern crate gauc;

use gauc::client::*;

fn main() {
    let mut client = Client::new("couchbase://localhost/default");

    fn cb_get(res: &str) {
        println!("{}", res);
    };

    client.get("foo", &cb_get);
}
