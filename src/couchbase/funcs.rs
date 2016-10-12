use libc::{c_char, c_void};

use super::types::callback_type::CallbackType;
use super::types::cmd_get::CmdGet;
use super::types::create_st::CreateSt;
use super::types::error_type::ErrorType;
use super::types::instance::Instance;
use super::types::response_base::ResponseBase;

pub type ResponseCallback = Option<unsafe extern "C" fn(instance: Instance, cbtype: CallbackType, resp: *const ResponseBase)>;

#[link(name = "couchbase")]
extern {
    pub fn lcb_create(instance: *mut Instance, options: *const CreateSt) -> ErrorType;
    pub fn lcb_connect(instance: Instance) -> ErrorType;
    pub fn lcb_wait(instance: Instance) -> ErrorType;
    pub fn lcb_get_bootstrap_status(instance: Instance) -> ErrorType;
    pub fn lcb_destroy(instance: Instance);
    pub fn lcb_strerror(instance: Instance, error: ErrorType) -> *const c_char;
    pub fn lcb_install_callback3(instance: Instance, cbtype: CallbackType, cb: ResponseCallback) -> ResponseCallback;
    pub fn lcb_get3(instance: Instance, cookie: *const c_void, cmd: *const CmdGet) -> ErrorType;
}
