extern crate log;
extern crate env_logger;

extern crate clap;
extern crate ctrlc;
extern crate gauc;
extern crate iron;
extern crate urlencoded;

use clap::{App, Arg};

use gauc::cli;
use gauc::web;
use gauc::client::Client;

use std::env;
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

const DESCRIPTION: &'static str = "Couchbase Rust Adapter / CLI / REST Interface";
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

//const DEFAULT_HOST: &'static str = "localhost";
//const DEFAULT_BUCKET: &'static str = "default";
const DEFAULT_URI: &'static str = "couchbase://localhost/default";

/// Handler of Ctrl+C
fn install_ctrl_c_handler() {
    let running = Arc::new(AtomicBool::new(true));
    let r = Arc::clone(&running);
    let _ = ctrlc::set_handler(move || {
        println!("");
        println!("");

        println!("Received ctrl+c, exiting ...");
        r.store(false, Ordering::SeqCst);
        exit(0);
    });
}

/// Web Server Entrypoint
fn main() {
    install_ctrl_c_handler();


    let couchbase_uri = match env::var("COUCHBASE_URI") {
        Ok(uri) => uri,
        Err(_) => DEFAULT_URI.to_string()
    };

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
            .help("URI - connection string")
            .short("u")
            .long("uri")
            .default_value(&couchbase_uri[..])
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

    if matches.is_present("interactive") {
        if let Ok(mut client) = Client::connect(matches.value_of("url").unwrap()) {
            cli::main(&matches, &mut client);
        }
    }

    let port: u16 = matches.value_of("rest-port").unwrap().to_string().parse::<u16>().unwrap();
    if matches.is_present("rest") {
        if let Ok(client) = Client::connect(matches.value_of("url").unwrap()) {
            web::start_web(&Arc::new(Mutex::new(client)), port);
        }
    }
}
