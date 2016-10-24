#[derive(Debug, Clone, Copy)]
pub enum InstanceInternal {}

pub type Instance = *mut InstanceInternal;
