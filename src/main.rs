extern crate clap;
extern crate gauc;

use clap::{App};
use gauc::client::*;

const DESCRIPTION: &'static str = "Couchbase Rust Adapter / CLI"; // env!("CARGO_PKG_DESCRIPTION");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    // Specify program options
    let _matches = App::new(DESCRIPTION)
        .version(VERSION)
        .author("Tomas Korcak <korczis@gmail.com>")
        .get_matches();

    let _client = Client::new("couchbase://localhost/default");
}
