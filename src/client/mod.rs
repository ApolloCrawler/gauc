mod client;
mod client2;

pub trait CouchbaseClient {
    // fn connect(url: &str) -> CouchbaseClient;
    fn get(&self, id: &str);
    fn store(&self, id: &str, value: &str);
}

pub use self::client::*;
pub use self::client2::*;
