use libc::{c_int};

use super::key_buffer::KeyBuffer;

#[repr(C)]
#[derive(Debug)]
pub struct CmdGet {
    pub cmdflags: u32,
    pub exptime: u32,
    pub cas: u64,
    pub key: KeyBuffer,
    pub hashkey: KeyBuffer,
    pub lock: c_int,
}

impl Default for CmdGet {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
