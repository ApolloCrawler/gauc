use super::super::super::client::Client;

/// Handle "info" command
pub fn cmd_info(client: &Client) -> bool {
    println!("{:?}", client);
    return true;
}
