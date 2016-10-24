#[macro_use]
extern crate log;
extern crate env_logger;

extern crate iron;
extern crate libc;
extern crate router;

pub mod cli;
pub mod client;
pub mod couchbase;
pub mod web;
