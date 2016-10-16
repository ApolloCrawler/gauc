extern crate clap;

use std::io;
use std::io::prelude::*;

use super::client::Client;

pub fn main(args: &clap::ArgMatches) {
    let mut client = Client::new(args.value_of("url").unwrap());

    let mut process_command = |cmd: &str| -> bool {
        let parts: Vec<&str> = cmd.split(' ').collect();
        match parts[0] {
            "exit" | "quit" => {
                return false;
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
                return true;
            },
            "info" => {
                println!("{:?}", client);
                return true;
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
                return true;
            },
            "" => {
                return true;
            },
            _ => {
                println!("Unknown command \"{}\"", cmd);
                return true;
            }
        }
    };

    let mut process_line = || -> bool {
        print!("gauc> ");
        let _ = io::stdout().flush();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                return process_command(&input.trim());
            }
            _ => {
                return false;
            }
        }
    };

    while process_line() {}
}
