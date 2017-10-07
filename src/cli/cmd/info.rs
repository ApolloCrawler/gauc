use super::super::super::client::Client;

/// Handle "info" command
pub fn cmd_info(client: &mut Client) -> bool {
    println!("{:?}", client);
    true
}
