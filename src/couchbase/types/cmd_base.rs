use super::key_buffer::KeyBuffer;

#[repr(C)]
#[derive(Debug)]
pub struct CmdBase {
    pub cmdflags: u32,
    pub exptime: u32,
    pub cas: u64,
    pub key: KeyBuffer,
    pub hashkey: KeyBuffer,
}

impl Default for CmdBase {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
