extern crate gauc;

use gauc::client::*;

fn main() {
    let mut client = Client::new("couchbase://localhost/default");

    fn cb_set(res: &str) {
        println!("Data stored.");
        println!("{}", res);
    };

    fn cb_get(res: &str) {
        println!("{}", res);
    };

    client.store("bar", "{\"msg\": \"This is test!\"}", &cb_set);

    // Get data - use function
    client.get("foo", &cb_get);

    // Get data - use in-place closure
    client.get("foo", |data: &str| println!("{}", data));
}
