#[macro_use]
extern crate log;
extern crate env_logger;

extern crate clap;
extern crate ctrlc;
extern crate gauc;
extern crate iron;

use clap::{App, Arg};

use gauc::cli;
use gauc::web;
use gauc::client::Client;

use std::env;
use std::process::exit;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

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

/// Web Server Entrypoint
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

    if matches.is_present("interactive") {
        let mut client = Client::new(matches.value_of("url").unwrap());
        cli::main(&matches, &mut client);
    }

    let url: String = matches.value_of("url").unwrap().clone().to_string();
    let port: u16 = matches.value_of("rest-port").unwrap().to_string().parse::<u16>().unwrap();
    if matches.is_present("rest") {
        let (tx, rx): (Sender<web::IronRequest>, Receiver<web::IronRequest>) = mpsc::channel();
        let shared_tx = std::sync::Arc::new(std::sync::Mutex::new(tx));

        println!("Talking to couchbase {:?}", url);
        let t = thread::spawn(move || {
            let mut client = Client::new(&url[..]);
            loop {
                let msg = rx.recv();
                debug!("Received {:?}", msg);
                match msg {
                    Ok(unwrapped_msg) => {
                        let doc = client.get_sync("foo");

                        let msg_res = web::IronResponse {
                            data: doc.unwrap().value.unwrap()
                        };

                        let _ = unwrapped_msg.tx.lock().unwrap().send(msg_res);
                    },
                    Err(_) => {}
                };
            }
        });

        web::start_web(port, shared_tx.clone());
        let _ = t.join();
    }
}
