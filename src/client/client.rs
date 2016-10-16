extern crate libc;

use libc::{c_void};
use std::ffi::CStr;
use std::ffi::CString;
use std::{process, ptr, thread, time};

use super::super::couchbase::*;

use super::super::couchbase::types::response::format_error;

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
    /// Constructor
    pub fn new(uri: &str) -> Client {
        let connstr = CString::new(uri).unwrap();

        let mut opts = CreateSt::default();
        opts.v3.connstr = connstr.as_ptr();

        let mut instance: Instance = ptr::null_mut();

        unsafe {
            let res = lcb_create(&mut instance as *mut Instance, &opts as *const CreateSt);
            if res != ErrorType::Success {
                error!("lcb_connect() failed - {:?}", res);
            }

            info!("Connecting to {}", uri);

            let res = lcb_connect(instance);
            if res != ErrorType::Success {
                error!("lcb_connect() failed - {:?}", res);
            }

            let res = lcb_wait(instance);
            if res != ErrorType::Success {
                error!("lcb_wait() failed - {:?}", res);
            }

            let res = lcb_get_bootstrap_status(instance);
            if res != ErrorType::Success {
                error!("lcb_get_bootstrap_status() failed - {:?}, \"{}\"",
                         res,
                         CStr::from_ptr(lcb_strerror(instance, res)).to_str().unwrap()
                );
                process::exit(-1);
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

    /// Get document from database
    pub fn get<F>(&mut self, key: &str, callback: F) -> &mut Client
        where F: Fn(Result<&response::Get, (Option<&response::Get>, &'static str)>)
    {
        let ckey = CString::new(key).unwrap();
        let mut gcmd = cmd::Get::default();
        gcmd.key._type = KvBufferType::Copy;
        gcmd.key.contig.bytes = ckey.as_ptr() as *const libc::c_void;
        gcmd.key.contig.nbytes = key.len() as u64;

        self.ops.total += 1;

        unsafe {
            let boxed: Box<Fn(&response::Get)> = Box::new(|response: &response::Get| {
                match response.rc {
                    ErrorType::Success => callback(Ok(response)),
                    _ => {
                        callback(Err((Some(response), response.error(self.instance))))
                    }
                }
            });

            let user_data = &boxed as *const _ as *mut c_void;

            let res = lcb_get3(self.instance, user_data, &gcmd as *const cmd::Get);
            if res != ErrorType::Success {
                callback(Err((None, format_error(self.instance, &res))))
            }

            let res = lcb_wait(self.instance);
            if res != ErrorType::Success {
                callback(Err((None, format_error(self.instance, &res))))
            }
        }

        return self;
    }

    /// Store document in database
    pub fn store<F>(&mut self, key: &str, value: &str, callback: F) -> &mut Client
        where F: Fn(Result<&response::Store, (Option<&response::Store>, &'static str)>)
    {
        let ckey = CString::new(key).unwrap();
        let cvalue = CString::new(value).unwrap();
        let mut gcmd = cmd::Store::default();
        gcmd.key._type = KvBufferType::Copy;
        gcmd.key.contig.bytes = ckey.as_ptr() as *const libc::c_void;
        gcmd.key.contig.nbytes = key.len() as u64;
        gcmd.value._type = KvBufferType::Copy;
        gcmd.value.contig.bytes = cvalue.as_ptr() as *const libc::c_void;
        gcmd.value.contig.nbytes = value.len() as u64;

        self.ops.total += 1;

        unsafe {
            let boxed: Box<Fn(&response::Store)> = Box::new(|response: &response::Store| {
                match response.rc {
                    ErrorType::Success => callback(Ok(response)),
                    _ => {
                        callback(Err((Some(response), response.error(self.instance))))
                    }
                }
            });

            let user_data = &boxed as *const _ as *mut c_void;

            let res = lcb_store3(self.instance, user_data, &gcmd as *const cmd::Store);
            if res != ErrorType::Success {
                callback(Err((None, format_error(self.instance, &res))))
            }

            let res = lcb_wait(self.instance);
            if res != ErrorType::Success {
                callback(Err((None, format_error(self.instance, &res))))
            }
        }

        return self;
    }

    /// Get count of finished commands
    pub fn ops_unfinished_count(&self) -> usize {
        return self.ops.total;
    }

    /// Get count of finished commands
    pub fn ops_finished(&mut self) -> bool {
        return self.ops_unfinished_count() == 0;
    }

    /// Wait for pending commands to finish
    pub fn wait(&mut self) {
         let interval = time::Duration::from_millis(100);
         while self.ops_finished() == false {
             thread::sleep(interval);
         }
    }

    /// Wait for pending commands to finish for max_msec
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
            info!("Disconnecting from {}", self.uri);
            lcb_destroy(self.instance);
        }
    }
}

/// libcouchbse callback
unsafe extern "C" fn op_callback(_instance: Instance, cbtype: CallbackType, resp: *const response::Base) {
    match cbtype {
        CallbackType::Get => {
            let gresp = resp as *const response::Get;
            debug!("{:?}", *gresp);

            let cookie = (*gresp).cookie;
            let callback = cookie as *const Box<Fn(&response::Get)>;
            (*callback)(&(*gresp));
        },
        CallbackType::Store => {
            let gresp = resp as *const response::Store;
            debug!("{:?}", *gresp);

            let cookie = (*gresp).cookie;
            let callback = cookie as *const Box<Fn(&response::Store)>;
            (*callback)(&(*gresp));
        },
        _ => error!("! Unknown Callback...")
    };
}
