#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum HandleType {
    Bucket = 0,
    Cluster = 1,
}
