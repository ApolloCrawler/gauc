extern crate clap;
extern crate gauc;

use clap::{App, Arg};
use gauc::cli;

const DESCRIPTION: &'static str = "Couchbase Rust Adapter / CLI";
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    // Specify program options
    let matches = App::new(DESCRIPTION)
        .version(VERSION)
        .author("Tomas Korcak <korczis@gmail.com>")
        .arg(Arg::with_name("interactive")
            .help("Interactive mode")
            .short("i")
            .long("interactive")
        )
        .arg(Arg::with_name("verbose")
            .help("Verbose mode")
            .short("v")
            .long("verbose")
            .multiple(true)
        )
        .get_matches();

    if matches.is_present("interactive") {
        cli::main(&matches);
    }
}
