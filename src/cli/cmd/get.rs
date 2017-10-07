use super::super::callback::get::get_callback;
use super::super::super::client::Client;

/// Handle "get" command
pub fn cmd_get(client: &mut Client, parts: &[&str]) -> bool {
    match parts.len() {
        2 => {
            // TODO: Add support for cas option
            client.get(parts[1], 0, get_callback);
        },
        _ => println!("Wrong number of arguments, expect exactly one argument.")
    }
    true
}
