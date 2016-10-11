use std::ffi::CString;
use std::ptr;
use std::ffi::CStr;

use super::super::couchbase::*;

pub struct Client {
    pub opts: LcbCreateSt,
    pub instance: LcbT
}

impl Client {
    pub fn new(uri: &str) -> Client {
        let connstr = CString::new(uri).unwrap();

        let mut opts = LcbCreateSt::default();
        opts.v3.connstr = connstr.as_ptr();

        let mut instance: LcbT = ptr::null_mut();

        unsafe {
            let res = lcb_create(&mut instance as *mut LcbT, &opts as *const LcbCreateSt);

            println!("Connecting to {}", uri);

            let res = lcb_connect(instance);
            let res = lcb_wait(instance);
            let res = lcb_get_bootstrap_status(instance);

            println!(
                "Bootstrap Status: {:?} \"{}\"",
                res,
                CStr::from_ptr(lcb_strerror(instance, res)).to_str().unwrap() // description
            );

            Client {
                opts: opts,
                instance: instance
            }
        }
    }
}
