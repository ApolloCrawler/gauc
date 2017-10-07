use std::ffi::CStr;

pub mod base;
pub mod flags;
pub mod get;
pub mod remove;
pub mod store;
pub mod view_query;

pub use self::base::*;
pub use self::flags::*;
pub use self::get::*;
pub use self::remove::*;
pub use self::store::*;
pub use self::view_query::*;

use super::super::error_type::ErrorType;
use super::super::instance::Instance;

use super::super::funcs::lcb_strerror;

pub fn format_error(instance: Instance, error: &ErrorType) -> &'static str {
    unsafe {
        CStr::from_ptr(lcb_strerror(instance, *error)).to_str().unwrap()
    }
}

