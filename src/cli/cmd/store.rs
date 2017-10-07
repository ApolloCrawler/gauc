use super::super::callback::store::store_callback;
use super::super::super::client::Client;
use super::super::super::couchbase::types::operation::Operation;

/// Handle "store" command
pub fn cmd_store(client: &mut Client, parts: &[&str]) -> bool {
    match parts.len() {
        1 | 2 => println!("Wrong number of arguments, expected key and value"),
        _ => {
            // TODO: Add support for cas option
            // TODO: Add support for exptime option
            client.store(parts[1], &format!("{}", parts[2..].join(" "))[..], Operation::Upsert, 0, 0, store_callback);
        }
    }

    true
}
