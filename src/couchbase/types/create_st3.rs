use libc::{c_char, c_void};

use super::handle_type::HandleType;

#[repr(C)]
#[derive(Debug)]
pub struct LcbCreateSt3 {
    pub connstr: *const c_char,
    pub username: *const c_char,
    pub passwd: *const c_char,
    _pad_bucket: *mut c_void,
    io: *mut c_void,
    pub _type: HandleType,
}

impl Default for LcbCreateSt3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
