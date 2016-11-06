#[macro_use]
extern crate log;
extern crate env_logger;

extern crate hyper;
extern crate iron;
extern crate libc;
extern crate router;
extern crate serde;
extern crate serde_json;

pub mod cli;
pub mod client;
pub mod couchbase;
pub mod web;
