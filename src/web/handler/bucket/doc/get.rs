use iron::prelude::*;
use iron::status;

use super::super::super::super::super::client::Client;

#[allow(dead_code)]
pub fn get_handler(_req: &mut Request, _client: &Client) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "ok\n")))
}
