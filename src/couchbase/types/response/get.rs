use std::ffi::CString;
use libc::{c_ulong, c_ulonglong, c_void};

use super::super::error_type::ErrorType;

#[repr(C)]
#[derive(Debug)]
pub struct Get {
    pub cookie: *mut c_void,
    pub key: *const c_void,
    pub nkey: c_ulong,
    pub cas: c_ulonglong,
    pub rc: ErrorType,
    pub version: u16,
    pub rflags: u16,
    pub value: *const c_void,
    pub nvalue: c_ulong,
    pub bufh: *mut c_void,
    pub datatype: u8,
    pub itmflags: u32,
}

impl Get {
    pub fn key(&self) -> String {
        unsafe {
            let res = CString::from_raw(self.key as *mut i8);
            let length = self.nkey as usize;

            let text = &res.into_string().unwrap()[..length];
            return text.to_string();
        }
    }

    pub fn value(&self) -> String {
        unsafe {
            let res = CString::from_raw(self.value as *mut i8);
            let length = self.nvalue as usize;

            let text = &res.into_string().unwrap()[..length];
            return text.to_string();
        }
    }
}
