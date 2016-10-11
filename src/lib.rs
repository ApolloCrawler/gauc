extern crate libc;

mod couchbase;

pub use couchbase::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
