use libc::{c_int};

use super::super::key_buffer::KeyBuffer;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Get {
    pub cmdflags: u32,
    pub exptime: u32,
    pub cas: u64,
    pub key: KeyBuffer,
    pub hashkey: KeyBuffer,

    pub lock: c_int,
}

impl Default for Get {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
