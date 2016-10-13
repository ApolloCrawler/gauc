extern crate clap;
extern crate env_logger;

use std::io;
use std::io::prelude::*;
use std::process;

use super::client::Client;

pub fn main(_args: &clap::ArgMatches) {
    env_logger::init().unwrap();

    let client = Client::new("couchbase://localhost/default");

    let process_command = |cmd: &str| {
        match cmd {
            "exit" | "quit" => {
                process::exit(0);
            },
            "get" => {
            },
            "info" => {
                println!("{:?}", client);
            },
            "" => {
            },
            _ => {
                println!("Unknown command \"{}\"", cmd);
            }
        }
    };

    let process_line = || {
        print!("gauc> ");
        let _ = io::stdout().flush();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                process_command(&input.trim());
            }
            _ => {}
        }
        return true;
    };

    while process_line() {}
}
