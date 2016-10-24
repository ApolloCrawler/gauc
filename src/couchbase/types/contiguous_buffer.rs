use libc::{c_ulong, c_void};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ContiguousBuffer {
    pub bytes: *const c_void,
    pub nbytes: c_ulong,
}
