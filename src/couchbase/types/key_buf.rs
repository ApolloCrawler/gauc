use super::contig_buf::LcbContigBuf;
use super::kv_buf_type::LcbKvBufType;

#[repr(C)]
#[derive(Debug)]
pub struct LcbKeyBuf {
    pub _type: LcbKvBufType,
    pub contig: LcbContigBuf,
}
