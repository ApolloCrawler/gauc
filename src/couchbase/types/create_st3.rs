use libc::{c_char, c_void};

use super::handle_type::HandleType;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CreateSt3 {
    pub connstr: *const c_char,
    pub username: *const c_char,
    pub passwd: *const c_char,
    pad_bucket: *mut c_void,
    io: *mut c_void,
    pub _type: HandleType,
}

unsafe impl Send for CreateSt3 {}
unsafe impl Sync for CreateSt3 {}

impl Default for CreateSt3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
