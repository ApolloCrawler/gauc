use libc::{c_ulong, c_ulonglong, c_void};

use super::super::error_type::ErrorType;
use super::super::instance::Instance;

use super::format_error;

#[repr(C)]
#[derive(Debug)]
pub struct GetInternal {
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

impl GetInternal {
    pub fn key(&self) -> Option<String> {
        unsafe {
            match self.rc {
                ErrorType::Success => {
                    let bytes = ::std::slice::from_raw_parts(self.key as *mut u8, self.nkey as usize);
                    let text = ::std::str::from_utf8(bytes).unwrap();

                    return Some(text.to_string());
                },
                _ => {
                    return None;
                }
            }
        }
    }

    pub fn value(&self) -> Option<String> {
        unsafe {
            match self.rc {
                ErrorType::Success => {
                    let bytes = ::std::slice::from_raw_parts(self.value as *mut u8, self.nvalue as usize);
                    let text = ::std::str::from_utf8(bytes).unwrap();
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

#[derive(Debug)]
pub struct Get {
    pub key: Option<String>,
    pub value: Option<String>,
    pub cas: u64,
    pub rc: ErrorType,
    pub version: u16,
    pub rflags: u16,
}

impl Get {
    pub fn new(internal: &GetInternal) -> Get {
        Get {
            key: internal.key(),
            value: internal.value(),
            cas: internal.cas,
            rc: internal.rc,
            version: internal.version,
            rflags: internal.rflags
        }
    }
}
