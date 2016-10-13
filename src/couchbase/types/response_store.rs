use libc::{c_ulong, c_ulonglong, c_void};

use super::error_type::ErrorType;
use super::operation::Operation;

#[repr(C)]
#[derive(Debug)]
pub struct ResponseStore {
    pub cookie: *mut c_void,
    pub key: *const c_void,
    pub nkey: c_ulong,
    pub cas: c_ulonglong,
    pub rc: ErrorType,
    pub version: u16,
    pub rflags: u16,
    pub operation: Operation
}
