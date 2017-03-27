use libc::{c_ulong, c_void};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ViewQuery {
    pub cmdflags: u32,

    pub ddoc: *const c_void,
    pub nddoc: c_ulong,

    pub view: *const c_void,
    pub nview: c_ulong,

    pub optstr: *const c_void,
    pub noptstr: c_ulong,

    pub postdata: *const c_void,
    pub npostdata: c_ulong,

    pub docs_concurrent_max: u64,

    pub callback: *mut c_void,
    pub handle: *mut c_void
}

impl Default for ViewQuery {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
