use libc::{c_char, c_void};

use super::lcb::LcbT;
use super::types::callback_type::LcbCallbackType;
use super::types::cmd_get::LcbCmdGet;
use super::types::error_type::LcbErrorT;
use super::types::create_st::LcbCreateSt;
use super::types::resp_base::LcbRespBase;

pub type LcbRespCallback = Option<unsafe extern "C" fn(instance: LcbT, cbtype: LcbCallbackType, resp: *const LcbRespBase)>;

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

