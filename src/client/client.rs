extern crate libc;

use libc::{c_void};
use std::ffi::CStr;
use std::ffi::CString;
use std::{fmt, process, ptr};
use std::collections::HashMap;
use std::mem;
use std::mem::{transmute};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

use std::sync::{Arc, Mutex};

use super::super::couchbase::*;

use super::super::couchbase::types::response::format_error;

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

pub struct CouchbaseOperation<T> {
    counter: u64,
    map: HashMap<u64, T>
}

impl<T> fmt::Debug for CouchbaseOperation<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CouchbaseOperation {{ counter: {}, length: {} }}", self.counter, self.map.len())
    }
}

impl<T> CouchbaseOperation<T> {
    pub fn new() -> CouchbaseOperation<T> {
        CouchbaseOperation {
            counter: 0,
            map: HashMap::new()
        }
    }

    pub fn increment_counter(&mut self) -> u64 {
        self.counter += 1;
        return self.counter;
    }
}

#[derive(Debug)]
pub struct CouchbaseOperations {
    get: CouchbaseOperation<cmd::Get>,
    store: CouchbaseOperation<cmd::Get>,
}

impl CouchbaseOperations {
    pub fn new() -> CouchbaseOperations {
        CouchbaseOperations {
            get: CouchbaseOperation::new(),
            store: CouchbaseOperation::new()
        }
    }
}

#[derive(Debug)]
pub struct Client {
    pub opts: Arc<Mutex<CreateSt>>,
    pub instance: Instance,
    pub uri: String,
    pub operations: CouchbaseOperations
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

            lcb_install_callback3(instance, CallbackType::Get, op_callback);
            lcb_install_callback3(instance, CallbackType::Store, op_callback);

            Client {
                opts: Arc::new(Mutex::new(opts)),
                instance: instance,
                uri: uri.to_string(),
                operations: CouchbaseOperations::new()

            }
        }
    }

    ///  Will cause the operation to fail if the key already exists in the cluster.
    pub fn add<'a, F>(&'a mut self, key: &str, value: &str, callback: F) -> &Client
        where F: Fn(OperationResultStore) + 'static
    {
        return self.store(key, value, Operation::Add, callback);
    }

    /// Rather than setting the contents of the entire document, take the value specified in value and _append_ it to the existing bytes in the value.
    pub fn append<'a, F>(&'a mut self, key: &str, value: &str, callback: F) -> &Client
        where F: Fn(OperationResultStore) + 'static
    {
        return self.store(key, value, Operation::Append, callback);
    }

    /// Get document from database
    pub fn get<'a, F>(&'a mut self, key: &str, callback: F) -> &Client
        where F: Fn(OperationResultGet) + 'static
    {
        let mut gcmd = cmd::Get::default();
        
        gcmd.key._type = KvBufferType::Copy;
        gcmd.key.contig.bytes = key.as_ptr() as *const libc::c_void;
        gcmd.key.contig.nbytes = key.len() as u64;
        
        unsafe {
            let _id = self.operations.get.increment_counter();

            let boxed: OperationResultGetInternalCallback = Box::new(Box::new(move |result: &response::GetInternal| {
                match result.rc {
                    ErrorType::Success => {
                        debug!("{:?}", result);
                        callback(Ok(response::Get::new(result)));
                    },
                    _ => {
                        callback(Err((Some(response::Get::new(result)), "error" /* result.error(self.instance) */)));
                    }
                }
            }));

            debug!("Boxed box occupies {} bytes in the stack", mem::size_of_val(&boxed));

            // let user_data = transmute::<*const _, *mut c_void>(Box::into_raw(boxed));
            let user_data = Box::into_raw(boxed) as *mut Box<Fn(&response::GetInternal)> as *mut c_void;
            debug!("Setting get callback address {:?}", user_data);

            let res = lcb_get3(self.instance, user_data, &gcmd as *const cmd::Get);
            if res != ErrorType::Success {
                // callback(Err((None, format_error(self.instance, &res))));
            } else {
                let res = lcb_wait(self.instance);
                if res != ErrorType::Success {
                    // callback(Err((None, format_error(self.instance, &res))))
                }
            }
        }

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

    /// Will cause the operation to fail _unless_ the key already exists in the cluster.
    pub fn replace<'a, F>(&'a mut self, key: &str, value: &str, callback: F) -> &Client
        where F: Fn(OperationResultStore) + 'static
    {
        return self.store(key, value, Operation::Replace, callback);
    }

    /// Unconditionally store the item in the cluster
    pub fn set<'a, F>(&'a mut self, key: &str, value: &str, callback: F) -> &Client
        where F: Fn(OperationResultStore) + 'static
    {
        return self.store(key, value, Operation::Set, callback);
    }

    /// Store document in database
    pub fn store<'a, F>(&'a mut self, key: &str, value: &str, operation: Operation, callback: F) -> &Client
        where F: Fn(OperationResultStore) + 'static
    {
        let mut gcmd = cmd::Store::default();
        gcmd.key._type = KvBufferType::Copy;
        gcmd.key.contig.bytes = key.as_bytes().as_ptr() as *const libc::c_void;
        gcmd.key.contig.nbytes = key.len() as u64;
        gcmd.value._type = KvBufferType::Copy;
        gcmd.value.contig.bytes = value.as_bytes().as_ptr() as *const libc::c_void;
        gcmd.value.contig.nbytes = value.len() as u64;
        gcmd.operation = operation;

        unsafe {
            let _id = self.operations.store.increment_counter();

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

            debug!("Boxed box occupies {} bytes in the stack", mem::size_of_val(&boxed));

            // let user_data = transmute::<*const  _, *mut c_void>(Box::into_raw(boxed));
            let user_data = Box::into_raw(boxed) as *mut Box<Fn(&response::StoreInternal)> as *mut c_void;
            debug!("Setting store callback address {:?}", user_data);

            let res = lcb_store3(self.instance, user_data, &gcmd as *const cmd::Store);
            if res != ErrorType::Success {
                // callback(Err((None, format_error(self.instance, &res))))
            } else {
                let res = lcb_wait(self.instance);
                if res != ErrorType::Success {
                    // callback(Err((None, format_error(self.instance, &res))))
                }
            }
        }

        
        return self;
    }

    /// Behaviorally it is identical to set in that it will make the server unconditionally store the item, whether it exists or not.
    pub fn upsert<'a, F>(&'a mut self, key: &str, value: &str, callback: F) -> &Client
        where F: Fn(OperationResultStore) + 'static
    {
        return self.store(key, value, Operation::Upsert, callback);
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
            let gresp = resp as *const response::GetInternal;
            debug!("{:?}", *gresp);

            let cookie = (*gresp).cookie;
            // let callback = transmute::<*mut c_void, *const Box<Fn(&response::GetInternal)>>(cookie);

            //let callback = cookie as *const Box<Fn(&response::GetInternal)>;
            let callback: Box<Box<Fn(&response::GetInternal)>> = Box::from_raw(cookie as *mut Box<Fn(&response::GetInternal)>);
            debug!("Retreived boxed box occupies {} bytes in the stack", mem::size_of_val(&callback));

            // debug!("Got get callback address {:?}", callback);

            (*callback)(&(*gresp));
        },
        CallbackType::Store => {
            let gresp = resp as *const response::StoreInternal;
            debug!("{:?}", *gresp);

            let cookie = (*gresp).cookie;
            // let callback = transmute::<*mut c_void, *const Box<Fn(&response::StoreInternal)>>(cookie);

            //let callback = cookie as *const Box<Fn(&response::StoreInternal)>;
            let callback: Box<Box<Fn(&response::StoreInternal)>> = Box::from_raw(cookie as *mut Box<Fn(&response::StoreInternal)>);
            debug!("Retreived boxed box occupies {} bytes in the stack", mem::size_of_val(&callback));

            // debug!("Got store callback address {:?}", callback);

            (*callback)(&(*gresp));
        },
        _ => error!("! Unknown Callback...")
    };
}
