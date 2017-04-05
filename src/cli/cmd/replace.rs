use super::super::callback::store::store_callback;
use super::super::super::client::Client;

/// Handle "set" command
pub fn cmd_replace(client: &mut Client, parts: &Vec<&str>) -> bool {
    match parts.len() {
        1 | 2 => println!("Wrong number of arguments, expected key and value"),
        _ => {
            // TODO: Add support for cas option
            // TODO: Add support for exptime option
            client.replace(parts[1], &format!("{}", parts[2..].join(" "))[..], 0, 0, store_callback);
        }
    }
    return true;
}
