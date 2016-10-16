extern crate clap;

use std::io;
use std::io::prelude::*;

use super::client::Client;
use super::couchbase::types::operation::Operation;

/// Handle "add" command
pub fn cmd_add(client: &mut Client, parts: &Vec<&str>) -> bool {
    match parts.len() {
        1 | 2 => println!("Wrong number of arguments, expected key and value"),
        _ => {
            client.add(parts[1], &format!("{}", parts[2..].join(" "))[..], |res| {
                match res {
                    Ok(response) => println!("{:?}", response),
                    Err(e) => {
                        let (_response, error) = e;
                        println!("{}", error);
                    }
                }
            });
        }
    }
    return true;
}

/// Handle "append" command
pub fn cmd_append(client: &mut Client, parts: &Vec<&str>) -> bool {
    match parts.len() {
        1 | 2 => println!("Wrong number of arguments, expected key and value"),
        _ => {
            client.append(parts[1], &format!("{}", parts[2..].join(" "))[..], |res| {
                match res {
                    Ok(response) => println!("{:?}", response),
                    Err(e) => {
                        let (_response, error) = e;
                        println!("{}", error);
                    }
                }
            });
        }
    }
    return true;
}

/// Handle empty command
pub fn cmd_empty() -> bool {
    return true;
}

/// Handle "exit" command
pub fn cmd_exit() -> bool {
    return false;
}

/// Handle "get" command
pub fn cmd_get(client: &mut Client, parts: &Vec<&str>) -> bool {
    match parts.len() {
        2 => {
            client.get(parts[1], |res| {
                match res {
                    Ok(response) => println!("{}", response.value().unwrap()),
                    Err(e) => {
                        let (_response, error) = e;
                        println!("{}", error);
                    }
                }

            });
        },
        _ => println!("Wrong number of arguments, expect exactly one argument.")
    }
    return true;
}

/// Handle "info" command
pub fn cmd_info(client: &mut Client) -> bool {
    println!("{:?}", client);
    return true;
}

/// Handle "prepend" command
pub fn cmd_prepend(client: &mut Client, parts: &Vec<&str>) -> bool {
    match parts.len() {
        1 | 2 => println!("Wrong number of arguments, expected key and value"),
        _ => {
            client.prepend(parts[1], &format!("{}", parts[2..].join(" "))[..], |res| {
                match res {
                    Ok(response) => println!("{:?}", response),
                    Err(e) => {
                        let (_response, error) = e;
                        println!("{}", error);
                    }
                }
            });
        }
    }
    return true;
}

/// Handle "replace" command
pub fn cmd_replace(client: &mut Client, parts: &Vec<&str>) -> bool {
    match parts.len() {
        1 | 2 => println!("Wrong number of arguments, expected key and value"),
        _ => {
            client.replace(parts[1], &format!("{}", parts[2..].join(" "))[..], |res| {
                match res {
                    Ok(response) => println!("{:?}", response),
                    Err(e) => {
                        let (_response, error) = e;
                        println!("{}", error);
                    }
                }
            });
        }
    }
    return true;
}

/// Handle "set" command
pub fn cmd_set(client: &mut Client, parts: &Vec<&str>) -> bool {
    match parts.len() {
        1 | 2 => println!("Wrong number of arguments, expected key and value"),
        _ => {
            client.set(parts[1], &format!("{}", parts[2..].join(" "))[..], |res| {
                match res {
                    Ok(response) => println!("{:?}", response),
                    Err(e) => {
                        let (_response, error) = e;
                        println!("{}", error);
                    }
                }
            });
        }
    }
    return true;
}

/// Handle "store" command
pub fn cmd_store(client: &mut Client, parts: &Vec<&str>) -> bool {
    match parts.len() {
        1 | 2 => println!("Wrong number of arguments, expected key and value"),
        _ => {
            client.store(parts[1], &format!("{}", parts[2..].join(" "))[..], Operation::Upsert, |res| {
                match res {
                    Ok(response) => println!("{:?}", response),
                    Err(e) => {
                        let (_response, error) = e;
                        println!("{}", error);
                    }
                }
            });
        }
    }
    return true;
}

/// Handle "upsert" command
pub fn cmd_upsert(client: &mut Client, parts: &Vec<&str>) -> bool {
    match parts.len() {
        1 | 2 => println!("Wrong number of arguments, expected key and value"),
        _ => {
            client.upsert(parts[1], &format!("{}", parts[2..].join(" "))[..], |res| {
                match res {
                    Ok(response) => println!("{:?}", response),
                    Err(e) => {
                        let (_response, error) = e;
                        println!("{}", error);
                    }
                }
            });
        }
    }
    return true;
}

/// Handle "unknown" command
pub fn cmd_unknown(cmd: &str) -> bool {
    println!("Unknown command \"{}\"", cmd);
    return true;
}

pub fn main(args: &clap::ArgMatches) {
    let mut client = Client::new(args.value_of("url").unwrap());

    let mut process_command = |cmd: &str| -> bool {
        let parts: Vec<&str> = cmd.split(' ').collect();
        match parts[0] {
            "add" => cmd_add(&mut client, &parts),
            "append" => cmd_append(&mut client, &parts),
            "exit" | "quit" => cmd_exit(),
            "get" => cmd_get(&mut client, &parts),
            "info" => cmd_info(&mut client),
            "prepend" => cmd_prepend(&mut client, &parts),
            "replace" => cmd_replace(&mut client, &parts),
            "set" => cmd_set(&mut client, &parts),
            "store" => cmd_store(&mut client, &parts),
            "upsert" => cmd_upsert(&mut client, &parts),
            "" => cmd_empty(),
            _ => cmd_unknown(cmd)
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
