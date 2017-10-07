extern crate clap;

mod callback;
mod cmd;
use std::io;
use std::io::prelude::*;

use super::client::Client;

pub fn main(_args: &clap::ArgMatches, mut client: &mut Client) {
    let mut process_command = |cmd: &str| -> bool {
        let parts: Vec<&str> = cmd.split(' ').collect();
        match parts[0] {
            "add" => cmd::add::cmd_add(&mut client, &parts),
            "append" => cmd::append::cmd_append(&mut client, &parts),
            "delete | remove" => cmd::remove::cmd_remove(&mut client, &parts),
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

        if io::stdin().read_line(&mut input).is_ok() {
            process_command(input.trim())
        } else {
            false
        }
    };

    while process_line() {}
}

