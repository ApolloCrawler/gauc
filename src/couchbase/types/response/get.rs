use libc::{c_ulong, c_ulonglong, c_void};
use std::fmt;

use super::super::error_type::ErrorType;
use super::super::instance::Instance;

use super::format_error;

#[repr(C)]
#[derive(Clone, Copy)]
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

impl fmt::Debug for GetInternal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "GetInternal {{ \
                cookie: {:?}, \
                key: {:?}, \
                cas: {:?}, \
                rc: {:?}, \
                version: {:?}, \
                rflags: {:?}, \
                value: {:?}, \
                bufh: {:?}, \
                datatype: {:?}, \
                itmflags: {:?} \
           }}",
            self.cookie,
            self.key(),
            self.cas,
            self.rc,
            self.version,
            self.rflags,
            self.value(),
            self.bufh,
            self.datatype,
            self.itmflags
        )
    }
}

impl GetInternal {
    pub fn key(&self) -> Option<String> {
        unsafe {
            match self.rc {
                ErrorType::Success => {
                    let bytes = ::std::slice::from_raw_parts(self.key as *mut u8, self.nkey as usize);
                    let text = ::std::str::from_utf8(bytes).unwrap();

                    Some(text.to_string())
                },
                _ => {
                    None
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
                    Some(text.to_string())
                },
                _ => {
                    None
                }
            }
        }
    }

    pub fn error(&self, instance: Instance) -> &'static str {
        format_error(instance, &self.rc)
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
