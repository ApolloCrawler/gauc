use libc::{c_ulong, c_ulonglong, c_void};
use std::{fmt};

use super::get;
use super::super::error_type::ErrorType;
use super::super::instance::Instance;

use super::format_error;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ViewQueryInternal {
    pub cookie: *mut c_void,
    pub key: *const c_void,
    pub nkey: c_ulong,
    pub cas: c_ulonglong,
    pub rc: ErrorType,
    pub version: u16,
    pub rflags: u16,
    pub docid: *const c_void,
    pub ndocid: c_ulong,
    pub value: *const c_void,
    pub nvalue: c_ulong,
    pub geometry: *const c_void,
    pub ngeometry: c_ulong,
    pub htresp: *const c_void,
    pub docresp: *const get::GetInternal
}

impl fmt::Debug for ViewQueryInternal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ViewQueryInternal {{ cookie: {:?}, \
                key: {:?}, \
                cas: {:?}, \
                rc: {:?}, \
                version: {:?}, \
                rflags: {:?}, \
                docid: {:?}, \
                value: {:?}, \
                geometry: {:?}, \
                httpres: {:?}, \
                docresp: {:?} \
           }}",
           self.cookie,
           self.key(),
           self.cas,
           self.rc,
           self.version,
           self.rflags,
           self.docid(),
           self.value(),
           self.geometry,
           self.htresp,
           self.docresp()
        )
    }
}

impl ViewQueryInternal {
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

    pub fn docid(&self) -> Option<String> {
        unsafe {
            match self.rc {
                ErrorType::Success => {
                    let bytes = ::std::slice::from_raw_parts(self.docid as *mut u8, self.ndocid as usize);
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

    pub fn geometry(&self) -> Option<String> {
        unsafe {
            match self.rc {
                ErrorType::Success => {
                    let bytes = ::std::slice::from_raw_parts(self.geometry as *mut u8, self.ngeometry as usize);
                    let text = ::std::str::from_utf8(bytes).unwrap();
                    Some(text.to_string())
                },
                _ => {
                    None
                }
            }
        }
    }

    pub fn docresp(&self) -> Option<get::GetInternal> {
        unsafe {
            if self.docresp.is_null() {
                None
            } else {
                Some(*self.docresp)
            }
        }
    }

    pub fn error(&self, instance: Instance) -> &'static str {
        format_error(instance, &self.rc)
    }
}

#[derive(Debug)]
pub struct ViewQuery {
    pub key: Option<String>,
    pub value: Option<String>,
    pub cas: u64,
    pub rc: ErrorType,
    pub version: u16,
    pub rflags: u16,
}

impl ViewQuery {
    pub fn new(internal: &ViewQueryInternal) -> ViewQuery {
        ViewQuery {
            key: internal.key(),
            value: internal.value(),
            cas: internal.cas,
            rc: internal.rc,
            version: internal.version,
            rflags: internal.rflags
        }
    }
}
