use super::CouchbaseClient;

#[derive(Debug)]
pub struct Client2 {
    pub uri: String,
    pub handle: u64
}

impl Client2 {
    pub fn new(uri: &str) -> Client2 {
        debug!("new({:?}) Connecting ...", uri);
        Client2 {
            uri: uri.to_string(),
            handle: 0
        }
    }
}

impl Clone for Client2 {
    fn clone(&self) -> Client2 {
        Client2 {
            uri: self.uri.clone(),
            handle: self.handle
        }
    }
}

impl CouchbaseClient for Client2 {
    fn get(&self, id: &str) {
        debug!("get({:?}) {:?}", id, self)
    }

    fn store(&self, id: &str, value: &str) {
        debug!("store({:?}, {:?}) {:?}", id, value, self)
    }
}
