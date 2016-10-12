#[repr(u32)]
#[derive(Debug,Clone,Copy)]
pub enum LcbRespFlags {
    LcbRespFFinal = 1,
    LcbRespFClientgen = 2,
    LcbRespFNmvgen = 4,
    LcbRespFExtdata = 8,
    LcbRespFSdsingle = 16,
}
