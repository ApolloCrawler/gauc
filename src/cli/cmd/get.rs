use super::super::get_callback;
use super::super::super::client::Client;

/// Handle "get" command
pub fn cmd_get(client: &mut Client, parts: &Vec<&str>) -> bool {
    match parts.len() {
        2 => {
            client.get(parts[1], get_callback);
        },
        _ => println!("Wrong number of arguments, expect exactly one argument.")
    }
    return true;
}
