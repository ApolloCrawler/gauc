use super::super::callback::store::store_callback;
use super::super::super::client::Client;

/// Handle "upsert" command
pub fn cmd_upsert(client: &mut Client, parts: &[&str]) -> bool {
    match parts.len() {
        1 | 2 => println!("Wrong number of arguments, expected key and value"),
        _ => {
            // TODO: Add support for cas option
            // TODO: Add support for exptime option
            client.upsert(parts[1], &format!("{}", parts[2..].join(" "))[..], 0, 0, store_callback);
        }
    }

    true
}
