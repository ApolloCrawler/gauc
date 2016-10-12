use libc::{c_int};

use super::key_buf::LcbKeyBuf;

#[repr(C)]
#[derive(Debug)]
pub struct LcbCmdGet {
    pub cmdflags: u32,
    pub exptime: u32,
    pub cas: u64,
    pub key: LcbKeyBuf,
    pub _hashkey: LcbKeyBuf,
    pub lock: c_int,
}

impl Default for LcbCmdGet {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
