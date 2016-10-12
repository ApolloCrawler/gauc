use libc::{c_ulong, c_ulonglong, c_void};

use super::error_type::LcbErrorT;

#[repr(C)]
#[derive(Debug)]
pub struct LcbRespBase {
    pub cookie: *mut c_void,
    pub key: *const c_void,
    pub nkey: c_ulong,
    pub cas: c_ulonglong,
    pub rc: LcbErrorT,
    pub version: u16,
    pub rflags: u16,
}
