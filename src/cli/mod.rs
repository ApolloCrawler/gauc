extern crate clap;
extern crate env_logger;

use std::io;
use std::io::prelude::*;
use std::process;

use super::client::Client;

pub fn main(_args: &clap::ArgMatches) {
    env_logger::init().unwrap();

    let mut client = Client::new("couchbase://localhost/default");

    let mut process_command = |cmd: &str| {
        let parts: Vec<&str> = cmd.split(' ').collect();
        match parts[0] {
            "exit" | "quit" => {
                process::exit(0);
            },
            "get" => {
                match parts.len() {
                    2 => {
                        client.get(parts[1], |res| {
                            match res {
                                Ok(response) => println!("{}", response.value().unwrap()),
                                Err(e) => println!("{}", e)
                            }

                        });
                    },
                    _ => println!("Wrong number of arguments, expect exactly one argument.")
                }
            },
            "info" => {
                println!("{:?}", client);
            },
            "store" => {
                match parts.len() {
                    1 | 2 => println!("Wrong number of arguments, expected key and value"),
                    _ => {
                        client.store(parts[1], &format!("{}", parts[2..].join(" "))[..], |res| {
                            match res {
                                Ok(response) => println!("{:?}", response),
                                Err(e) => println!("{}", e)
                            }
                        });
                    }
                }
            },
            "" => {
            },
            _ => {
                println!("Unknown command \"{}\"", cmd);
            }
        }
    };

    let mut process_line = || {
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
