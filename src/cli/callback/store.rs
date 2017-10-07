use super::super::super::couchbase::types::error_type::ErrorType;
use super::super::super::couchbase::types::response::Store;

/// Callback handling store operation
pub fn store_callback(result: Result<Store, (Option<Store>, ErrorType)>) {
    match result {
        Ok(response) => {
            println!("{:?}", response);
        },
        Err(e) => {
            let (_response, error) = e;
            println!("{}", error);
        }
    }
}
