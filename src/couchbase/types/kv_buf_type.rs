#[repr(u32)]
#[derive(Debug)]
pub enum LcbKvBufType {
    LcbKvCopy = 0,
    LcbKvContig = 1,
    LcbKvIov = 2,
    LcbKvVbid = 3,
    LcbKvIovcopy = 4,
}
