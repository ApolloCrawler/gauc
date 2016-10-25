use super::super::key_buffer::KeyBuffer;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Base {
    pub cmdflags: u32,
    pub exptime: u32,
    pub cas: u64,
    pub key: KeyBuffer,
    pub hashkey: KeyBuffer,
}

impl Default for Base {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
