use super::super::callback::store::store_callback;
use super::super::super::client::Client;

/// Handle "set" command
pub fn cmd_replace(client: &mut Client, parts: &Vec<&str>) -> bool {
    match parts.len() {
        1 | 2 => println!("Wrong number of arguments, expected key and value"),
        _ => {
            client.replace(parts[1], &format!("{}", parts[2..].join(" "))[..], store_callback);
        }
    }
    return true;
}
