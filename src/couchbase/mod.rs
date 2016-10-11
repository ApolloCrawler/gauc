use libc::{c_int, c_char, c_void, c_ulong, c_ulonglong};

use std::ffi::CStr;
use std::fmt;
use std::ptr;

#[repr(u32)]
#[derive(Debug,Clone,Copy)]
pub enum LcbTypeT {
    LcbTypeBucket = 0,
    LcbTypeCluster = 1,
}

#[repr(u32)]
#[derive(Debug,Clone,Copy)]
pub enum LcbErrorT {
    LcbSuccess = 0,
    LcbAuthContinue = 1,
    LcbAuthError = 2,
    LcbDeltaBadval = 3,
    LcbE2big = 4,
    LcbEbusy = 5,
    LcbEinternal = 6,
    LcbEinval = 7,
    LcbEnomem = 8,
    LcbErange = 9,
    LcbError = 10,
    LcbEtmpfail = 11,
    LcbKeyEexists = 12,
    LcbKeyEnoent = 13,
    LcbDlopenFailed = 14,
    LcbDlsymFailed = 15,
    LcbNetworkError = 16,
    LcbNotMyVbucket = 17,
    LcbNotStored = 18,
    LcbNotSupported = 19,
    LcbUnknownCommand = 20,
    LcbUnknownHost = 21,
    LcbProtocolError = 22,
    LcbEtimedout = 23,
    LcbConnectError = 24,
    LcbBucketEnoent = 25,
    LcbClientEnomem = 26,
    LcbClientEnoconf = 27,
    LcbEbadhandle = 28,
    LcbServerBug = 29,
    LcbPluginVersionMismatch = 30,
    LcbInvalidHostFormat = 31,
    LcbInvalidChar = 32,
    LcbDurabilityEtoomany = 33,
    LcbDuplicateCommands = 34,
    LcbNoMatchingServer = 35,
    LcbBadEnvironment = 36,
    LcbBusy = 37,
    LcbInvalidUsername = 38,
    LcbConfigCacheInvalid = 39,
    LcbSaslmechUnavailable = 40,
    LcbTooManyRedirects = 41,
    LcbMapChanged = 42,
    LcbIncompletePacket = 43,
    LcbEconnrefused = 44,
    LcbEsockshutdown = 45,
    LcbEconnreset = 46,
    LcbEcantgetport = 47,
    LcbEfdlimitreached = 48,
    LcbEnetunreach = 49,
    LcbEctlUnknown = 50,
    LcbEctlUnsuppmode = 51,
    LcbEctlBadarg = 52,
    LcbEmptyKey = 53,
    LcbSslError = 54,
    LcbSslCantverify = 55,
    LcbSchedfailInternal = 56,
    LcbClientFeatureUnavailable = 57,
    LcbOptionsConflict = 58,
    LcbHttpError = 59,
    LcbDurabilityNoMutationTokens = 60,
    LcbUnknownMemcachedError = 61,
    LcbMutationLost = 62,
    LcbSubdocPathEnoent = 63,
    LcbSubdocPathMismatch = 64,
    LcbSubdocPathEinval = 65,
    LcbSubdocPathE2big = 66,
    LcbSubdocDocE2deep = 67,
    LcbSubdocValueCantinsert = 68,
    LcbSubdocDocNotjson = 69,
    LcbSubdocNumErange = 70,
    LcbSubdocBadDelta = 71,
    LcbSubdocPathEexists = 72,
    LcbSubdocMultiFailure = 73,
    LcbSubdocValueE2deep = 74,
    LcbEinvalMcd = 75,
    LcbEmptyPath = 76,
    LcbUnknownSdcmd = 77,
    LcbEnoCommands = 78,
    LcbQueryError = 79,
    LcbMaxError = 4096,
}

impl fmt::Display for LcbErrorT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = unsafe {
            CStr::from_ptr(lcb_strerror(ptr::null_mut(), *self)).to_str().unwrap()
        };
        write!(f,"{} ({:?})", description, self)
    }
}

pub enum LcbSt { }
pub type LcbT = *mut LcbSt;

#[repr(C)]
#[derive(Debug)]
pub struct LcbCreateSt3 {
    pub connstr: *const c_char,
    pub username: *const c_char,
    pub passwd: *const c_char,
    _pad_bucket: *mut c_void,
    io: *mut c_void,
    pub _type: LcbTypeT,
}

impl Default for LcbCreateSt3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

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

#[repr(C)]
#[derive(Debug)]
pub struct LcbRespBase {
    pub cookie: *mut c_void,
    pub key: *const c_void,
    pub nkey: c_ulong,
    pub cas: c_ulonglong,
    pub rc: LcbErrorT,
    pub version: u16,
    pub rflags: u16,
}

#[repr(C)]
#[derive(Debug)]
pub struct LcbRespGet {
    pub cookie: *mut c_void,
    pub key: *const c_void,
    pub nkey: c_ulong,
    pub cas: c_ulonglong,
    pub rc: LcbErrorT,
    pub version: u16,
    pub rflags: u16,
    pub value: *const c_void,
    pub nvalue: c_ulong,
    pub bufh: *mut c_void,
    pub datatype: u8,
    pub itmflags: u32,
}

#[repr(u32)]
#[derive(Debug,Clone,Copy)]
pub enum LcbCallbackType {
    LcbCallbackDefault = 0,
    LcbCallbackGet = 1,
    LcbCallbackStore = 2,
    LcbCallbackCounter = 3,
    LcbCallbackTouch = 4,
    LcbCallbackRemove = 5,
    LcbCallbackUnlock = 6,
    LcbCallbackStats = 7,
    LcbCallbackVersions = 8,
    LcbCallbackVerbosity = 9,
    LcbCallbackFlush = 10,
    LcbCallbackObserve = 11,
    LcbCallbackGetreplica = 12,
    LcbCallbackEndure = 13,
    LcbCallbackHttp = 14,
    LcbCallbackCbflush = 15,
    LcbCallbackObseqno = 16,
    LcbCallbackStoredur = 17,
    LcbCallbackSdlookup = 18,
    LcbCallbackSdmutate = 19,
    LcbCallbackMax = 20,
}

pub type LcbRespCallback = Option<unsafe extern "C" fn(instance: LcbT, cbtype: LcbCallbackType, resp: *const LcbRespBase)>;

#[repr(u32)]
#[derive(Debug,Clone,Copy)]
pub enum LcbRespFlags {
    LcbRespFFinal = 1,
    LcbRespFClientgen = 2,
    LcbRespFNmvgen = 4,
    LcbRespFExtdata = 8,
    LcbRespFSdsingle = 16,
}

#[repr(u32)]
#[derive(Debug)]
pub enum LcbKvBufType {
    LcbKvCopy = 0,
    LcbKvContig = 1,
    LcbKvIov = 2,
    LcbKvVbid = 3,
    LcbKvIovcopy = 4,
}

#[repr(C)]
#[derive(Debug)]
pub struct LcbKeyBuf {
    pub _type: LcbKvBufType,
    pub contig: LcbContigBuf,
}

#[repr(C)]
#[derive(Debug)]
pub struct LcbContigBuf {
    pub bytes: *const c_void,
    pub nbytes: c_ulong,
}

#[repr(C)]
#[derive(Debug)]
pub struct LcbCmdGet {
    pub cmdflags: u32,
    pub exptime: u32,
    pub cas: u64,
    pub key: LcbKeyBuf,
    pub _hashkey: LcbKeyBuf,
    pub lock: c_int,
}

impl Default for LcbCmdGet {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[link(name = "couchbase")]
extern {
    pub fn lcb_create(instance: *mut LcbT, options: *const LcbCreateSt) -> LcbErrorT;
    pub fn lcb_connect(instance: LcbT) -> LcbErrorT;
    pub fn lcb_wait(instance: LcbT) -> LcbErrorT;
    pub fn lcb_get_bootstrap_status(instance: LcbT) -> LcbErrorT;
    pub fn lcb_destroy(instance: LcbT);
    pub fn lcb_strerror(instance: LcbT, error: LcbErrorT) -> *const c_char;
    pub fn lcb_install_callback3(instance: LcbT, cbtype: LcbCallbackType, cb: LcbRespCallback) -> LcbRespCallback;
    pub fn lcb_get3(instance: LcbT, cookie: *const c_void, cmd: *const LcbCmdGet) -> LcbErrorT;
}
