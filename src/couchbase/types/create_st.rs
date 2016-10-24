use libc::{c_int};

use super::create_st3::CreateSt3;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CreateSt {
    version: c_int,
    pub v3: CreateSt3,
}

impl Default for CreateSt {
    fn default() -> Self {
        CreateSt {
            version: 3,
            v3: CreateSt3::default()
        }
    }
}

impl CreateSt {
    pub fn version(&self) -> c_int {
        return self.version;
    }
}
