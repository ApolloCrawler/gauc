use super::key_buffer::KeyBuffer;

use super::operation::Operation;

#[repr(C)]
#[derive(Debug)]
pub struct CmdStore {
    pub cmdflags: u32,
    pub exptime: u32,
    pub cas: u64,
    pub key: KeyBuffer,
    pub hashkey: KeyBuffer,

    /*
     * Value to store on the server.
     */
    pub value: KeyBuffer,

    /**
     * Format flags used by clients to determine the underlying encoding of
     * the value.
     */
    pub flags: u32,

    /** Do not set this value for now */
    pub datatype: u8,


    /** Controls *how* the operation is perfomed. See the documentation for
     * operation::Operation
     */
    pub operation: Operation,
}

impl Default for CmdStore {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
