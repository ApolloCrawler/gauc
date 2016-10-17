use super::super::callback::store::store_callback;
use super::super::super::client::Client;

/// Handle "append" command
pub fn cmd_append(client: &Client, parts: &Vec<&str>) -> bool {
    match parts.len() {
        1 | 2 => println!("Wrong number of arguments, expected key and value"),
        _ => {
            client.append(parts[1], &format!("{}", parts[2..].join(" "))[..], store_callback);
        }
    }
    return true;
}
