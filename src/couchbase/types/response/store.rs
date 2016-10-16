use std::ffi::CString;
use libc::{c_ulong, c_ulonglong, c_void};

use super::super::error_type::ErrorType;
use super::super::operation::Operation;
use super::super::instance::Instance;

use super::format_error;

#[repr(C)]
#[derive(Debug)]
pub struct Store {
    pub cookie: *mut c_void,
    pub key: *const c_void,
    pub nkey: c_ulong,
    pub cas: c_ulonglong,
    pub rc: ErrorType,
    pub version: u16,
    pub rflags: u16,
    pub operation: Operation
}

impl Store {
    pub fn key(&self) -> Option<String> {
        unsafe {
            match self.rc {
                ErrorType::Success => {
                    let res = CString::from_raw(self.key as *mut i8);
                    let length = self.nkey as usize;

                    let text = &res.into_string().unwrap()[..length];
                    return Some(text.to_string());
                },
                _ => {
                    return None;
                }
            }
        }
    }

    pub fn error(&self, instance: Instance) -> &'static str {
        return format_error(instance, &self.rc);
    }
}
