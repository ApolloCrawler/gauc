extern crate libc;

use libc::{c_void};
use std::ffi::CStr;
use std::ffi::CString;
use std::{process};
use std::mem::{forget};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

use std::sync::{Arc, Mutex};

use super::super::couchbase::*;

// Gets
pub type OperationResultGet = Result<response::Get, (Option<response::Get>, &'static str)>;
pub type OperationResultGetCallback = Box<Box<Fn(&response::Get)>>;
pub type OperationResultGetInternal<'a> = Result<&'a response::GetInternal, (Option<&'a response::GetInternal>, &'static str)>;
pub type OperationResultGetInternalCallback = Box<Box<Fn(&response::GetInternal)>>;

// Stores
pub type OperationResultStore = Result<response::Store, (Option<response::Store>, &'static str)>;
pub type OperationResultStoreCallback = Box<Box<Fn(&response::Store)>>;
pub type OperationResultStoreInternal<'a> = Result<&'a response::StoreInternal, (Option<&'a response::StoreInternal>, &'static str)>;
pub type OperationResultStoreInternalCallback = Box<Box<Fn(&response::StoreInternal)>>;

#[derive(Clone, Debug)]
pub struct Client {
    pub opts: Option<Arc<Mutex<CreateSt>>>,
    pub instance: Option<Arc<Mutex<Instance>>>,
    pub uri: Option<String>
}

impl Client {
    pub fn new() -> Client {
        Client {
            opts: None,
            instance: None,
            uri: None
        }
    }

    /// Constructor
    pub fn connect(&mut self, uri: &str) {
        let connstr = CString::new(uri).unwrap();

        let mut opts = CreateSt::default();
        opts.v3.connstr = connstr.as_ptr();

        let mut instance: Instance = Instance::default();

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

            lcb_install_callback3(instance, CallbackType::Get, op_callback);
            lcb_install_callback3(instance, CallbackType::Store, op_callback);

            self.opts = Some(Arc::new(Mutex::new(opts)));
            self.instance = Some(Arc::new(Mutex::new(instance)));
            self.uri = Some(uri.to_string());

        }
    }

    ///  Will cause the operation to fail if the key already exists in the cluster.
    pub fn add<'a, F>(&'a mut self, key: &str, value: &str, callback: F) -> &Client
        where F: Fn(OperationResultStore) + 'static
    {
        return self.store(key, value, Operation::Add, callback);
    }

    pub fn add_sync(&mut self, key: &str, value: &str) -> OperationResultStore
    {
        return self.store_sync(key, value, Operation::Add);
    }

    /// Rather than setting the contents of the entire document, take the value specified in value and _append_ it to the existing bytes in the value.
    pub fn append<'a, F>(&'a mut self, key: &str, value: &str, callback: F) -> &Client
        where F: Fn(OperationResultStore) + 'static
    {
        return self.store(key, value, Operation::Append, callback);
    }

    pub fn append_sync(&mut self, key: &str, value: &str) -> OperationResultStore
    {
        return self.store_sync(key, value, Operation::Append);
    }

    /// Get document from database
    pub fn get<'a, F>(&'a mut self, key: &str, callback: F) -> &Client
        where F: Fn(OperationResultGet) + 'static
    {
        let key = key.to_owned();

        let mut gcmd = cmd::Get::default();

        gcmd.key._type = KvBufferType::Copy;
        gcmd.key.contig.bytes = key.as_ptr() as *const libc::c_void;
        gcmd.key.contig.nbytes = key.len() as u64;

        unsafe {
            let instance = self.instance.as_ref().unwrap().lock().unwrap();

            let boxed: OperationResultGetInternalCallback = Box::new(Box::new(move |result: &response::GetInternal| {
                match result.rc {
                    ErrorType::Success => {
                        debug!("{:?}", result);
                        callback(Ok(response::Get::new(result)));
                    },
                    _ => {
                        callback(Err((Some(response::Get::new(result)), "error" /* result.error(*instance) */)));
                    }
                }
            }));

            let user_data = Box::into_raw(boxed) as *mut Box<Fn(&response::GetInternal)> as *mut c_void;

            let res = lcb_get3(*instance, user_data, &gcmd as *const cmd::Get);
            if res != ErrorType::Success {
                // callback(Err((None, format_error(*instance, &res))));
            } else {
                let res = lcb_wait(*instance);
                if res != ErrorType::Success {
                    // callback(Err((None, format_error(*instance, &res))))
                }
            }
        }

        forget(key);

        return self;
    }

    pub fn get_sync(&mut self, key: &str) -> OperationResultGet
    {
        let (tx, rx): (Sender<OperationResultGet>, Receiver<OperationResultGet>) = mpsc::channel();
        self.get(key, move |result: OperationResultGet| {
            let _ = tx.send(result);
        });
        return rx.recv().unwrap();
    }

    /// Like append, but prepends the new value to the existing value.
    pub fn prepend<'a, F>(&'a mut self, key: &str, value: &str, callback: F) -> &Client
        where F: Fn(OperationResultStore) + 'static
    {
        return self.store(key, value, Operation::Prepend, callback);
    }

    pub fn prepend_sync(&mut self, key: &str, value: &str) -> OperationResultStore
    {
        return self.store_sync(key, value, Operation::Prepend);
    }

    /// Will cause the operation to fail _unless_ the key already exists in the cluster.
    pub fn replace<'a, F>(&'a mut self, key: &str, value: &str, callback: F) -> &Client
        where F: Fn(OperationResultStore) + 'static
    {
        return self.store(key, value, Operation::Replace, callback);
    }

    pub fn replace_sync(&mut self, key: &str, value: &str) -> OperationResultStore
    {
        return self.store_sync(key, value, Operation::Replace);
    }

    /// Unconditionally store the item in the cluster
    pub fn set<'a, F>(&'a mut self, key: &str, value: &str, callback: F) -> &Client
        where F: Fn(OperationResultStore) + 'static
    {
        return self.store(key, value, Operation::Set, callback);
    }

    pub fn set_sync(&mut self, key: &str, value: &str) -> OperationResultStore
    {
        return self.store_sync(key, value, Operation::Set);
    }

    /// Store document in database
    pub fn store<'a, F>(&'a mut self, key: &str, value: &str, operation: Operation, callback: F) -> &Client
        where F: Fn(OperationResultStore) + 'static
    {
        let key = key.to_owned();

        let mut gcmd = cmd::Store::default();
        gcmd.key._type = KvBufferType::Copy;
        gcmd.key.contig.bytes = key.as_bytes().as_ptr() as *const libc::c_void;
        gcmd.key.contig.nbytes = key.len() as u64;
        gcmd.value._type = KvBufferType::Copy;
        gcmd.value.contig.bytes = value.as_bytes().as_ptr() as *const libc::c_void;
        gcmd.value.contig.nbytes = value.len() as u64;
        gcmd.operation = operation;

        unsafe {
            let boxed: OperationResultStoreInternalCallback = Box::new(Box::new(move |result: &response::StoreInternal| {
                match result.rc {
                    ErrorType::Success => {
                        debug!("{:?}", result);
                        callback(Ok(response::Store::new(result)));
                    },
                    _ => {
                        callback(Err((Some(response::Store::new(result)), "error" /* result.error(self.instance) */)));
                    }
                }
            }));

            let user_data = Box::into_raw(boxed) as *mut Box<Fn(&response::StoreInternal)> as *mut c_void;

            let instance = self.instance.as_ref().unwrap().lock().unwrap();
            let res = lcb_store3(*instance, user_data, &gcmd as *const cmd::Store);
            if res != ErrorType::Success {
                // callback(Err((None, format_error(instance, &res))))
            } else {
                let res = lcb_wait(*instance);
                if res != ErrorType::Success {
                    // callback(Err((None, format_error(instance, &res))))
                }
            }
        }

        return self;
    }

    pub fn store_sync(&mut self, key: &str, value: &str, operation: Operation) -> OperationResultStore
    {
        let (tx, rx): (Sender<OperationResultStore>, Receiver<OperationResultStore>) = mpsc::channel();
        self.store(key, value, operation, move |result: OperationResultStore| {
            let _ = tx.send(result);
        });
        return rx.recv().unwrap();
    }

    /// Behaviorally it is identical to set in that it will make the server unconditionally store the item, whether it exists or not.
    pub fn upsert<'a, F>(&'a mut self, key: &str, value: &str, callback: F) -> &Client
        where F: Fn(OperationResultStore) + 'static
    {
        return self.store(key, value, Operation::Upsert, callback);
    }

    pub fn upsert_sync(&mut self, key: &str, value: &str) -> OperationResultStore
    {
        return self.store_sync(key, value, Operation::Upsert);
    }
}

//impl Clone for Client {
//
//}

impl Drop for Client {
    fn drop(&mut self) {
        unsafe {
            info!("Disconnecting from {}", self.uri.as_ref().unwrap());
            let instance = self.instance.as_ref().unwrap().lock().unwrap();
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
            let callback: Box<Box<Fn(&response::GetInternal)>> = Box::from_raw(cookie as *mut Box<Fn(&response::GetInternal)>);
            (*callback)(&(*gresp));
        },
        CallbackType::Store => {
            let gresp = resp as *const response::StoreInternal;
            debug!("{:?}", *gresp);

            let cookie = (*gresp).cookie;
            let callback: Box<Box<Fn(&response::StoreInternal)>> = Box::from_raw(cookie as *mut Box<Fn(&response::StoreInternal)>);
            (*callback)(&(*gresp));
        },
        _ => error!("! Unknown Callback...")
    };
}
