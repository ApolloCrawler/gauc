use libc::{c_char, c_void};

use super::lcb::LcbT;
use super::types::callback_type::CallbackType;
use super::types::cmd_get::CmdGet;
use super::types::error_type::ErrorType;
use super::types::create_st::CreateSt;
use super::types::response_base::ResponseBase;

pub type LcbRespCallback = Option<unsafe extern "C" fn(instance: LcbT, cbtype: CallbackType, resp: *const ResponseBase)>;

#[link(name = "couchbase")]
extern {
    pub fn lcb_create(instance: *mut LcbT, options: *const CreateSt) -> ErrorType;
    pub fn lcb_connect(instance: LcbT) -> ErrorType;
    pub fn lcb_wait(instance: LcbT) -> ErrorType;
    pub fn lcb_get_bootstrap_status(instance: LcbT) -> ErrorType;
    pub fn lcb_destroy(instance: LcbT);
    pub fn lcb_strerror(instance: LcbT, error: ErrorType) -> *const c_char;
    pub fn lcb_install_callback3(instance: LcbT, cbtype: CallbackType, cb: LcbRespCallback) -> LcbRespCallback;
    pub fn lcb_get3(instance: LcbT, cookie: *const c_void, cmd: *const CmdGet) -> ErrorType;
}
