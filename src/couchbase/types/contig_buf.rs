use libc::{c_ulong, c_void};

#[repr(C)]
#[derive(Debug)]
pub struct LcbContigBuf {
    pub bytes: *const c_void,
    pub nbytes: c_ulong,
}
