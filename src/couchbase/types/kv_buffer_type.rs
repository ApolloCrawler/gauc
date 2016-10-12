#[repr(u32)]
#[derive(Debug)]
pub enum KvBufferType {
    Copy = 0,
    Contig = 1,
    Iov = 2,
    Vbid = 3,
    Iovcopy = 4,
}
