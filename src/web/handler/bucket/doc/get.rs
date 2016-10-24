use iron::prelude::*;
use iron::status;

use super::super::super::super::super::client::Client;

#[allow(dead_code)]
pub fn get_handler(_req: &mut Request, client: &mut Client) -> IronResult<Response> {
    match client.get_sync("foo1") {
        res => {
            let data = res.unwrap().value;
            println!("{:?}", data);
            Ok(Response::with((status::Ok, "")))
        }
    }
}
