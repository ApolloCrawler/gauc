use super::super::super::couchbase::types::error_type::ErrorType;
use super::super::super::couchbase::types::response::Get;

/// Callback handling get operation
pub fn get_callback(result: Result<Get, (Option<Get>, ErrorType)>) {
    match result {
        Ok(response) => {
            println!("{}", response.value.unwrap());
        },
        Err(e) => {
            let (_response, error) = e;
            println!("{}", error);
        }
    }
}
