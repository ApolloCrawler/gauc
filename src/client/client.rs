extern crate libc;

use std::ffi::CStr;
use std::ffi::CString;
use std::{ptr, thread, time};

use super::super::couchbase::*;

#[derive(Debug)]
pub struct ClientOps {
    pub total: usize
}

#[derive(Debug)]
pub struct Client {
    pub opts: LcbCreateSt,
    pub instance: LcbT,
    pub uri: String,
    pub ops: ClientOps
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

            /* let res = */ lcb_connect(instance);
            /* let res = */ lcb_wait(instance);
            /* let res = */ lcb_get_bootstrap_status(instance);

            println!(
                "Bootstrap Status: {:?} \"{}\"",
                res,
                CStr::from_ptr(lcb_strerror(instance, res)).to_str().unwrap() // description
            );

            lcb_install_callback3(instance, LcbCallbackType::LcbCallbackGet, Some(op_callback));

            let ops = ClientOps {
                total: 0
            };

            Client {
                opts: opts,
                instance: instance,
                uri: uri.to_string(),
                ops: ops
            }
        }
    }

    pub fn get(&mut self, key: &str) -> &mut Client {
        println!("Getting document with id \"{}\"", key);
        let ckey = CString::new("foo").unwrap();
        let mut gcmd = LcbCmdGet::default();
        gcmd.key._type = LcbKvBufType::LcbKvCopy;
        gcmd.key.contig.bytes = ckey.as_ptr() as *const libc::c_void;

        self.ops.total += 1;

        unsafe {
            let res = lcb_get3(self.instance, ptr::null(), &gcmd as *const LcbCmdGet);
            println!("Get Res: {:?}", res);

            let res = lcb_wait(self.instance);
            println!("Get Wait Res: {:?}", res);
        }

        self
    }

    pub fn ops_unfinished_count(&self) -> usize {
        return self.ops.total;
    }

    pub fn ops_finished(&mut self) -> bool {
        return self.ops_unfinished_count() == 0;
    }

    pub fn wait(&mut self) {
         let interval = time::Duration::from_millis(100);
         while self.ops_finished() == false {
             thread::sleep(interval);
         }
    }

    pub fn wait_max(&mut self, max_msec: usize) {
        let mut t = 0 as usize;
        let interval = time::Duration::from_millis(100);
        while self.ops_finished() == false {
            thread::sleep(interval);
            t += 100;

            if t > max_msec {
                println!("wait_max - still {} operations unfinished", self.ops_unfinished_count());
                break;
            }
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        unsafe {
            println!("Disconnecting from {}", self.uri);
            lcb_destroy(self.instance);
        }
    }
}

unsafe extern "C" fn op_callback(_instance: LcbT, cbtype: LcbCallbackType, resp: *const LcbRespBase) {
    match cbtype {
        LcbCallbackType::LcbCallbackGet => {
            println!("> Get Callback!");
            let gresp = resp as *const LcbRespGet;
            println!(">> CAS: {}", (*gresp).cas);

            if  (*gresp).value.is_null() == false {
                let res = CString::from_raw((*gresp).value as *mut i8);
                let length = (*gresp).nvalue as usize;

                println!(">> Content: {}", &res.into_string().unwrap()[..length]);
            }
        },
        _ => panic!("! Unknown Callback...")
    };
}
