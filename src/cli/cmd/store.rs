use super::super::store_callback;
use super::super::super::client::Client;
use super::super::super::couchbase::types::operation::Operation;

/// Handle "store" command
pub fn cmd_store(client: &mut Client, parts: &Vec<&str>) -> bool {
    match parts.len() {
        1 | 2 => println!("Wrong number of arguments, expected key and value"),
        _ => {
            client.store(parts[1], &format!("{}", parts[2..].join(" "))[..], Operation::Upsert, store_callback);
        }
    }
    return true;
}
