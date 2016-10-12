#[repr(u32)]
#[derive(Debug,Clone,Copy)]
pub enum HandleType {
    LcbTypeBucket = 0,
    LcbTypeCluster = 1,
}
