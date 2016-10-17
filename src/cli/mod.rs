extern crate clap;

mod cmd;

use std::io;
use std::io::prelude::*;

use super::client::Client;

use super::couchbase::types::response::Get;
use super::couchbase::types::response::Store;

/// Callback handling get operation
fn get_callback(result: Result<&Get, (Option<&Get>, &'static str)>) {
    match result {
        Ok(response) => println!("{}", response.value().unwrap()),
        Err(e) => {
            let (_response, error) = e;
            println!("{}", error);
        }
    }

}

/// Callback handling store operation
fn store_callback(result: Result<&Store, (Option<&Store>, &'static str)>) {
    match result {
        Ok(response) => println!("{:?}", response),
        Err(e) => {
            let (_response, error) = e;
            println!("{}", error);
        }
    }
}

pub fn main(args: &clap::ArgMatches) {
    let mut client = Client::new(args.value_of("url").unwrap());

    let mut process_command = |cmd: &str| -> bool {
        let parts: Vec<&str> = cmd.split(' ').collect();
        match parts[0] {
            "add" => cmd::add::cmd_add(&mut client, &parts),
            "append" => cmd::append::cmd_append(&mut client, &parts),
            "exit" | "quit" => cmd::exit::cmd_exit(),
            "get" => cmd::get::cmd_get(&mut client, &parts),
            "info" => cmd::info::cmd_info(&mut client),
            "prepend" => cmd::prepend::cmd_prepend(&mut client, &parts),
            "replace" => cmd::replace::cmd_replace(&mut client, &parts),
            "set" => cmd::set::cmd_set(&mut client, &parts),
            "store" => cmd::store::cmd_store(&mut client, &parts),
            "upsert" => cmd::upsert::cmd_upsert(&mut client, &parts),
            "" => cmd::empty::cmd_empty(),
            _ => cmd::unknown::cmd_unknown(cmd)
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
