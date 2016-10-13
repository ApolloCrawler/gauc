extern crate libc;

use libc::{c_void};
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
    pub opts: CreateSt,
    pub instance: Instance,
    pub uri: String,
    pub ops: ClientOps
}

impl Client {
    pub fn new(uri: &str) -> Client {
        let connstr = CString::new(uri).unwrap();

        let mut opts = CreateSt::default();
        opts.v3.connstr = connstr.as_ptr();

        let mut instance: Instance = ptr::null_mut();

        unsafe {
            let res = lcb_create(&mut instance as *mut Instance, &opts as *const CreateSt);
            if res != ErrorType::Success {
                println!("lcb_connect() failed - {:?}", res);
            }

            println!("Connecting to {}", uri);

            let res = lcb_connect(instance);
            if res != ErrorType::Success {
                println!("lcb_connect() failed - {:?}", res);
            }

            let res = lcb_wait(instance);
            if res != ErrorType::Success {
                println!("lcb_wait() failed - {:?}", res);
            }

            let res = lcb_get_bootstrap_status(instance);
            if res != ErrorType::Success {
                println!("lcb_get_bootstrap_status() failed - {:?}, \"{}\"",
                         res,
                         CStr::from_ptr(lcb_strerror(instance, res)).to_str().unwrap()
                );
            }

            lcb_install_callback3(instance, CallbackType::Get, Some(op_callback));
            lcb_install_callback3(instance, CallbackType::Store, Some(op_callback));

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

    pub fn get<F>(&mut self, key: &str, callback: F) -> &mut Client
        where F: Fn(&str)
    {
        let ckey = CString::new(key).unwrap();
        let mut gcmd = CmdGet::default();
        gcmd.key._type = KvBufferType::Copy;
        gcmd.key.contig.bytes = ckey.as_ptr() as *const libc::c_void;
        gcmd.key.contig.nbytes = key.len() as u64;

        self.ops.total += 1;

        unsafe {
            let boxed: Box<Fn(&str)> = Box::new(|res: &str| {
                callback(res);
            });

            let user_data = &boxed as *const _ as *mut c_void;

            let res = lcb_get3(self.instance, user_data, &gcmd as *const CmdGet);
            if res != ErrorType::Success {
                println!("lcb_get3() failed - {:?}", res);
            }

            let res = lcb_wait(self.instance);
            if res != ErrorType::Success {
                println!("lcb_wait() failed - {:?}", res);
            }
        }

        self
    }

    pub fn store<F>(&mut self, key: &str, value: &str, callback: F) -> &mut Client
        where F: Fn(&str)
    {
        let ckey = CString::new(key).unwrap();
        let cvalue = CString::new(value).unwrap();
        let mut gcmd = CmdStore::default();
        gcmd.key._type = KvBufferType::Copy;
        gcmd.key.contig.bytes = ckey.as_ptr() as *const libc::c_void;
        gcmd.key.contig.nbytes = key.len() as u64;
        gcmd.value._type = KvBufferType::Copy;
        gcmd.value.contig.bytes = cvalue.as_ptr() as *const libc::c_void;
        gcmd.value.contig.nbytes = value.len() as u64;

        unsafe {
            let boxed: Box<Fn(&str)> = Box::new(|res: &str| {
                callback(res);
            });

            let user_data = &boxed as *const _ as *mut c_void;

            let res = lcb_store3(self.instance, user_data, &gcmd as *const CmdStore);
            if res != ErrorType::Success {
                println!("lcb_get3() failed - {:?}", res);
            }

            let res = lcb_wait(self.instance);
            if res != ErrorType::Success {
                println!("lcb_wait() failed - {:?}", res);
            }
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

unsafe extern "C" fn op_callback(_instance: Instance, cbtype: CallbackType, resp: *const ResponseBase) {
    match cbtype {
        CallbackType::Get => {
            let gresp = resp as *const ResponseGet;

            if  (*gresp).value.is_null() == false {
                let res = CString::from_raw((*gresp).value as *mut i8);
                let length = (*gresp).nvalue as usize;

                let text = &res.into_string().unwrap()[..length];
                let cookie = (*gresp).cookie;
                let callback = cookie as *const Box<Fn(&str)>;
                (*callback)(text);
            }
        },
        CallbackType::Store => {
           let gresp = resp as *const ResponseStore;

            let cookie = (*gresp).cookie;
            let callback = cookie as *const Box<Fn(&str)>;
            (*callback)("Response");
        },
        _ => panic!("! Unknown Callback...")
    };
}
