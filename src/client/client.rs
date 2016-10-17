extern crate libc;

use libc::{c_void};
use std::ffi::CStr;
use std::ffi::CString;
use std::{process, ptr};

use std::sync::{Arc, Mutex};

use super::super::couchbase::*;

use super::super::couchbase::types::response::format_error;

#[derive(Debug)]
pub struct Client {
    pub opts: Arc<Mutex<CreateSt>>,
    pub instance: Arc<Mutex<Instance>>,
    pub uri: String
}

type OperationResultGet = Result<response::Get, (Option<response::Get>, &'static str)>;
type OperationResultGetInternal<'a> = Result<&'a response::GetInternal, (Option<&'a response::GetInternal>, &'static str)>;

type OperationResultStore = Result<response::Store, (Option<response::Store>, &'static str)>;
type OperationResultStoreInternal<'a> = Result<&'a response::StoreInternal, (Option<&'a response::StoreInternal>, &'static str)>;

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

            Client {
                opts: Arc::new(Mutex::new(opts)),
                instance: Arc::new(Mutex::new(instance)),
                uri: uri.to_string()
            }
        }
    }

    ///  Will cause the operation to fail if the key already exists in the cluster.
    pub fn add<'a, F>(&'a self, key: &str, value: &str, callback: F) -> &Client
        where F: Fn(OperationResultStore)
    {
        return self.store(key, value, Operation::Add, callback);
    }

    /// Rather than setting the contents of the entire document, take the value specified in value and _append_ it to the existing bytes in the value.
    pub fn append<'a, F>(&self, key: &str, value: &str, callback: F) -> &Client
        where F: Fn(OperationResultStore)
    {
        return self.store(key, value, Operation::Append, callback);
    }

    /// Get document from database
    pub fn get<'a, F>(&'a self, key: &str, callback: F) -> &Client
        where F: Fn(OperationResultGet)
    {
        let ckey = CString::new(key).unwrap();
        let mut gcmd = cmd::Get::default();

        gcmd.key._type = KvBufferType::Copy;
        gcmd.key.contig.bytes = ckey.as_ptr() as *const libc::c_void;
        gcmd.key.contig.nbytes = key.len() as u64;

        unsafe {
            let boxed: Box<Fn(&response::GetInternal)> = Box::new(|response: &response::GetInternal| {
                match response.rc {
                    ErrorType::Success => callback(Ok(response::Get::new(response))),
                    _ => {
                        let instance = self.instance.lock().unwrap();
                        callback(Err((Some(response::Get::new(response)), response.error(*instance))));
                    }
                }
            });

            let user_data = &boxed as *const _ as *mut c_void;

            let instance = self.instance.lock().unwrap();
            let res = lcb_get3(*instance, user_data, &gcmd as *const cmd::Get);
            if res != ErrorType::Success {
                callback(Err((None, format_error(*instance, &res))));
            } else {
                let res = lcb_wait(*instance);
                if res != ErrorType::Success {
                    callback(Err((None, format_error(*instance, &res))))
                }
            }
        }

        return self;
    }

    /// Like append, but prepends the new value to the existing value.
    pub fn prepend<'a, F>(&'a self, key: &str, value: &str, callback: F) -> &Client
        where F: Fn(OperationResultStore)
    {
        return self.store(key, value, Operation::Prepend, callback);
    }

    /// Will cause the operation to fail _unless_ the key already exists in the cluster.
    pub fn replace<'a, F>(&'a self, key: &str, value: &str, callback: F) -> &Client
        where F: Fn(OperationResultStore)
    {
        return self.store(key, value, Operation::Replace, callback);
    }

    /// Unconditionally store the item in the cluster
    pub fn set<'a, F>(&'a self, key: &str, value: &str, callback: F) -> &Client
        where F: Fn(OperationResultStore)
    {
        return self.store(key, value, Operation::Set, callback);
    }

    /// Store document in database
    pub fn store<'a, F>(&'a self, key: &str, value: &str, operation: Operation, callback: F) -> &Client
        where F: Fn(OperationResultStore)
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
        gcmd.operation = operation;

        unsafe {
            let boxed: Box<Fn(&response::StoreInternal)> = Box::new(|response: &response::StoreInternal| {
                match response.rc {
                    ErrorType::Success => callback(Ok(response::Store::new(response))),
                    _ => {
                        let instance = self.instance.lock().unwrap();
                        callback(Err((Some(response::Store::new(response)), response.error(*instance))));
                    }
                }
            });

            let user_data = &boxed as *const _ as *mut c_void;

            let instance = self.instance.lock().unwrap();
            let res = lcb_store3(*instance, user_data, &gcmd as *const cmd::Store);
            if res != ErrorType::Success {
                callback(Err((None, format_error(*instance, &res))))
            } else {
                let res = lcb_wait(*instance);
                if res != ErrorType::Success {
                    callback(Err((None, format_error(*instance, &res))))
                }
            }
        }

        return self;
    }

    /// Behaviorally it is identical to set in that it will make the server unconditionally store the item, whether it exists or not.
    pub fn upsert<'a, F>(&'a self, key: &str, value: &str, callback: F) -> &Client
        where F: Fn(OperationResultStore)
    {
        return self.store(key, value, Operation::Upsert, callback);
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        unsafe {
            info!("Disconnecting from {}", self.uri);
            let instance = self.instance.lock().unwrap();
            lcb_destroy(*instance);
        }
    }
}

/// libcouchbse callback
unsafe extern "C" fn op_callback(_instance: Instance, cbtype: CallbackType, resp: *const response::Base) {
    match cbtype {
        CallbackType::Get => {
            let gresp = resp as *const response::GetInternal;
            debug!("{:?}", *gresp);

            let cookie = (*gresp).cookie;
            let callback = cookie as *const Box<Fn(&response::GetInternal)>;
            (*callback)(&(*gresp));
        },
        CallbackType::Store => {
            let gresp = resp as *const response::StoreInternal;
            debug!("{:?}", *gresp);

            let cookie = (*gresp).cookie;
            let callback = cookie as *const Box<Fn(&response::StoreInternal)>;
            (*callback)(&(*gresp));
        },
        _ => error!("! Unknown Callback...")
    };
}
