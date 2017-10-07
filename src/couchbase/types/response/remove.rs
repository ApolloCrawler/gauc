use libc::{c_ulong, c_ulonglong, c_void};

use super::super::error_type::ErrorType;
use super::super::instance::Instance;

use super::format_error;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RemoveInternal {
    pub cookie: *mut c_void,
    pub key: *const c_void,
    pub nkey: c_ulong,
    pub cas: c_ulonglong,
    pub rc: ErrorType,
    pub version: u16,
    pub rflags: u16
}

impl RemoveInternal {
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

    pub fn error(&self, instance: Instance) -> &'static str {
        format_error(instance, &self.rc)
    }
}

#[derive(Debug)]
pub struct Remove {
    pub key: Option<String>,
    pub cas: u64,
    pub rc: ErrorType,
    pub version: u16,
    pub rflags: u16,
}

impl Remove {
    pub fn new(internal: &RemoveInternal) -> Remove {
        Remove {
            key: internal.key(),
            cas: internal.cas,
            rc: internal.rc,
            version: internal.version,
            rflags: internal.rflags
        }
    }
}
