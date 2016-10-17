extern crate clap;

mod callback;
mod cmd;
use std::io;
use std::io::prelude::*;

use super::client::Client;

pub fn main(_args: &clap::ArgMatches, client: &Client) {
    let process_command = |cmd: &str| -> bool {
        let parts: Vec<&str> = cmd.split(' ').collect();
        match parts[0] {
            "add" => cmd::add::cmd_add(&client, &parts),
            "append" => cmd::append::cmd_append(&client, &parts),
            "exit" | "quit" => cmd::exit::cmd_exit(),
            "get" => cmd::get::cmd_get(&client, &parts),
            "info" => cmd::info::cmd_info(&client),
            "prepend" => cmd::prepend::cmd_prepend(&client, &parts),
            "replace" => cmd::replace::cmd_replace(&client, &parts),
            "set" => cmd::set::cmd_set(&client, &parts),
            "store" => cmd::store::cmd_store(&client, &parts),
            "upsert" => cmd::upsert::cmd_upsert(&client, &parts),
            "" => cmd::empty::cmd_empty(),
            _ => cmd::unknown::cmd_unknown(cmd)
        }
    };

    let process_line = || -> bool {
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
