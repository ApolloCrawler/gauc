use libc::{c_int};

use super::create_st3::LcbCreateSt3;

#[repr(C)]
#[derive(Debug)]
pub struct LcbCreateSt {
    version: c_int,
    pub v3: LcbCreateSt3,
}

impl Default for LcbCreateSt {
    fn default() -> Self {
        LcbCreateSt { version: 3, v3: LcbCreateSt3::default() }
    }
}
