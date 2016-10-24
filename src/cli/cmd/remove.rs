use super::super::callback::get::get_callback;
use super::super::super::client::Client;

/// Handle "remove" command
pub fn cmd_remove(client: &mut Client, parts: &Vec<&str>) -> bool {
    match parts.len() {
        2 => {
            client.get(parts[1], get_callback);
        },
        _ => println!("Wrong number of arguments, expect exactly one argument.")
    }
    return true;
}
