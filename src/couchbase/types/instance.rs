#[derive(Debug, Clone, Copy)]
pub enum InstanceInternal {}

unsafe impl Send for InstanceInternal {}
unsafe impl Sync for InstanceInternal {}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct InstancePtr {
    ptr: *mut InstanceInternal
}


impl Default for InstancePtr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl Send for InstancePtr {}
unsafe impl Sync for InstancePtr {}

pub type Instance = InstancePtr;
