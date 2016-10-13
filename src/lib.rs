#[macro_use]
extern crate log;

extern crate libc;

pub mod cli;
pub mod client;
pub mod couchbase;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
