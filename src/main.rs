extern crate clap;
extern crate ctrlc;
extern crate env_logger;
extern crate gauc;

use clap::{App, Arg};

use gauc::cli;
use gauc::web;
use gauc::client::Client;

use std::env;
use std::process::exit;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

const DESCRIPTION: &'static str = "Couchbase Rust Adapter / CLI / REST Interface";
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// Handler of Ctrl+C
fn install_ctrl_c_handler() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        println!("");
        println!("");

        println!("Received ctrl+c, exiting ...");
        r.store(false, Ordering::SeqCst);
        exit(0);
    });
}

///
fn main() {
    install_ctrl_c_handler();

    // Specify program options
    let matches = App::new(DESCRIPTION)
        .version(VERSION)
        .author("Tomas Korcak <korczis@gmail.com>")
        .arg(Arg::with_name("interactive")
            .help("Interactive mode")
            .short("i")
            .long("interactive")
        )
        .arg(Arg::with_name("rest")
            .help("Run REST Server")
            .short("r")
            .long("rest")
        )
        .arg(Arg::with_name("rest-port")
            .help("REST Port")
            .short("p")
            .long("rest-port")
            .default_value("5000")
        )
        .arg(Arg::with_name("url")
            .help("URL - connection string")
            .short("u")
            .long("url")
            .default_value("couchbase://localhost/default")
        )
        .arg(Arg::with_name("verbose")
            .help("Verbose mode")
            .short("v")
            .long("verbose")
            .multiple(true)
        )
        .get_matches();

    match matches.occurrences_of("verbose") {
        1 => env::set_var("RUST_LOG", "warn"),
        2 => env::set_var("RUST_LOG", "info"),
        3 => env::set_var("RUST_LOG", "debug"),
        _ => {}
    }

    env_logger::init().unwrap();

    let mut client = Client::new(matches.value_of("url").unwrap());

    if matches.is_present("rest") {
        web::start_web(&matches, &mut client);
    }

    if matches.is_present("interactive") {
        cli::main(&matches, &mut client);
    }
}
