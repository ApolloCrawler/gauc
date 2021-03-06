extern crate libc;

use libc::{c_void};
use std::ffi::CStr;
use std::ffi::CString;
use std::mem::{forget};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

use std::result;

use super::couchbase::*;

// Gets
pub type OperationResultGet = Result<response::Get, (Option<response::Get>, types::error_type::ErrorType)>;
pub type OperationResultGetCallback = Box<Box<Fn(&response::Get)>>;
pub type OperationResultGetInternal<'a> = Result<&'a response::GetInternal, (Option<&'a response::GetInternal>, types::error_type::ErrorType)>;
pub type OperationResultGetInternalCallback = Box<Box<Fn(&response::GetInternal)>>;

// Remove
pub type OperationResultRemove = Result<response::Remove, (Option<response::Remove>, types::error_type::ErrorType)>;
pub type OperationResultRemoveCallback = Box<Box<Fn(&response::Remove)>>;
pub type OperationResultRemoveInternal<'a> = Result<&'a response::RemoveInternal, (Option<&'a response::RemoveInternal>, types::error_type::ErrorType)>;
pub type OperationResultRemoveInternalCallback = Box<Box<Fn(&response::RemoveInternal)>>;

// Store
pub type OperationResultStore = Result<response::Store, (Option<response::Store>, types::error_type::ErrorType)>;
pub type OperationResultStoreCallback = Box<Box<Fn(&response::Store)>>;
pub type OperationResultStoreInternal<'a> = Result<&'a response::StoreInternal, (Option<&'a response::StoreInternal>, types::error_type::ErrorType)>;
pub type OperationResultStoreInternalCallback = Box<Box<Fn(&response::StoreInternal)>>;

// ViewQuery
pub type OperationResultViewQuery = Result<response::ViewQuery, (Option<response::ViewQuery>, types::error_type::ErrorType)>;
pub type OperationResultViewQueryCallback = Box<Box<Fn(&response::ViewQuery)>>;
pub type OperationResultViewQueryInternal<'a> = Result<&'a response::ViewQueryInternal, (Option<&'a response::ViewQueryInternal>, types::error_type::ErrorType)>;
pub type OperationResultViewQueryInternalCallback = Box<Box<Fn(&response::ViewQueryInternal)>>;
pub type OperationResultViewQueryInternalRowCallback = Box<Box<Fn(&Instance, &u64, *mut c_void)>>;

#[derive(Debug)]
pub struct Client {
    pub opts: CreateSt,
    pub instance: Instance,
    pub uri: String
}

impl Clone for Client {
    fn clone(&self) -> Client {
        let uri = &self.uri.clone()[..];
        Client::connect(uri).unwrap()
    }
}

impl Client {
    pub fn connect(uri: &str) -> result::Result<Client, String> {
        let connstr = CString::new(uri).unwrap();

        let mut opts = CreateSt::default();
        opts.v3.connstr = connstr.as_ptr();

        let mut instance: Instance = Instance::default();

        unsafe {
            let res = lcb_create(&mut instance as *mut Instance, &opts as *const CreateSt);
            if res != ErrorType::Success {
                let str = CStr::from_ptr(lcb_strerror(instance, res)).to_str().unwrap().to_string();
                return Err(format!("lcb_create() - {}", &str));
            }

            info!("Connecting to {}", uri);

            let res = lcb_connect(instance);
            if res != ErrorType::Success {
                let str = CStr::from_ptr(lcb_strerror(instance, res)).to_str().unwrap().to_string();
                return Err(format!("lcb_connect() - {}", &str));
            }

            let key = CString::new("error_thresh_delay").unwrap();
            let value = CString::new("5.0").unwrap();

            // http://docs.couchbase.com/sdk-api/couchbase-c-client-2.5.6/group__lcb-cntl.html#gab3df573dbbea79cfa8ce77f6f61563dc
            lcb_cntl_string(instance,
                            key.as_ptr(),
                            value.as_ptr()
            );

            let res = lcb_wait(instance);
            if res != ErrorType::Success {
                let str = CStr::from_ptr(lcb_strerror(instance, res)).to_str().unwrap().to_string();
                return Err(format!("lcb_wait() - {}", &str));
            }

            let res = lcb_get_bootstrap_status(instance);
            if res != ErrorType::Success {
                let str = CStr::from_ptr(lcb_strerror(instance, res)).to_str().unwrap().to_string();
                return Err(format!("lcb_get_bootstrap_status() - {}", &str));
            }

            lcb_install_callback3(instance, CallbackType::Get, op_callback);
            lcb_install_callback3(instance, CallbackType::Remove, op_callback);
            lcb_install_callback3(instance, CallbackType::Store, op_callback);

            Ok(Client {
                opts,
                instance,
                uri: uri.to_string()
            })
        }
    }

    ///  Will cause the operation to fail if the key already exists in the cluster.
    pub fn add<'a, F>(&'a mut self, key: &str, value: &str, cas: u64, exptime: u32, callback: F) -> &Client
        where F: Fn(OperationResultStore) + 'static
    {
        self.store(key, value, Operation::Add, cas, exptime, callback)
    }

    pub fn add_sync(&mut self, key: &str, value: &str, cas: u64, exptime: u32) -> OperationResultStore
    {
        self.store_sync(key, value, Operation::Add, cas, exptime)
    }

    /// Rather than setting the contents of the entire document, take the value specified in value and _append_ it to the existing bytes in the value.
    pub fn append<'a, F>(&'a mut self, key: &str, value: &str, cas: u64, exptime: u32, callback: F) -> &Client
        where F: Fn(OperationResultStore) + 'static
    {
        self.store(key, value, Operation::Append, cas, exptime, callback)
    }

    pub fn append_sync(&mut self, key: &str, value: &str, cas: u64, exptime: u32) -> OperationResultStore
    {
        self.store_sync(key, value, Operation::Append, cas, exptime)
    }

    /// Get document from database
    pub fn get<'a, F>(&'a mut self, key: &str, cas: u64, callback: F) -> &Client
        where F: Fn(OperationResultGet) + 'static
    {
        let key = key.to_owned();

        let mut gcmd = cmd::Get::default();
        gcmd.cas = cas;
        gcmd.key._type = KvBufferType::Copy;
        gcmd.key.contig.bytes = key.as_ptr() as *const libc::c_void;
        gcmd.key.contig.nbytes = key.len() as u64;

        unsafe {
            let boxed: OperationResultGetInternalCallback = Box::new(Box::new(move |result: &response::GetInternal| {
                match result.rc {
                    ErrorType::Success => {
                        debug!("{:?}", result);
                        callback(Ok(response::Get::new(result)));
                    },
                    e => {
                        // let _ = format_error(self.instance, &e);
                        callback(Err((Some(response::Get::new(result)), e)));
                    }
                }
            }));

            let user_data = Box::into_raw(boxed) as *mut Box<Fn(&response::GetInternal)> as *mut c_void;

            let res = lcb_get3(self.instance, user_data, &gcmd as *const cmd::Get);
            if res != ErrorType::Success {
                error!("lcb_get3() failed");
                // callback(Err((None, format_error(self.instance, &res))));
            } else if lcb_wait(self.instance) != ErrorType::Success {
                error!("lcb_wait() failed");
                // callback(Err((None, format_error(self.instance, &res))))
            }
        }

        forget(key);

        self
    }

    pub fn get_sync(&mut self, key: &str, cas: u64) -> OperationResultGet
    {
        let (tx, rx): (Sender<OperationResultGet>, Receiver<OperationResultGet>) = mpsc::channel();
        self.get(key, cas, move |result: OperationResultGet| {
            let _ = tx.send(result);
        });

        rx.recv().unwrap()
    }

    /// Like append, but prepends the new value to the existing value.
    pub fn prepend<'a, F>(&'a mut self, key: &str, value: &str, cas: u64, exptime: u32, callback: F) -> &Client
        where F: Fn(OperationResultStore) + 'static
    {
        self.store(key, value, Operation::Prepend, cas, exptime, callback)
    }

    pub fn prepend_sync(&mut self, key: &str, value: &str, cas: u64, exptime: u32) -> OperationResultStore
    {
        self.store_sync(key, value, Operation::Prepend, cas, exptime)
    }

    /// Remove document from database
    pub fn remove<'a, F>(&'a mut self, key: &str, callback: F) -> &Client
        where F: Fn(OperationResultRemove) + 'static
    {
        let key = key.to_owned();

        let mut gcmd = cmd::Remove::default();

        gcmd.key._type = KvBufferType::Copy;
        gcmd.key.contig.bytes = key.as_ptr() as *const libc::c_void;
        gcmd.key.contig.nbytes = key.len() as u64;

        unsafe {
            let boxed: OperationResultRemoveInternalCallback = Box::new(Box::new(move |result: &response::RemoveInternal| {
                match result.rc {
                    ErrorType::Success => {
                        debug!("{:?}", result);
                        callback(Ok(response::Remove::new(result)));
                    },
                    e => {
                        callback(Err((Some(response::Remove::new(result)), e)));
                    }
                }
            }));

            let user_data = Box::into_raw(boxed) as *mut Box<Fn(&response::RemoveInternal)> as *mut c_void;

            let res = lcb_remove3(self.instance, user_data, &gcmd as *const cmd::Remove);
            if res != ErrorType::Success {
                error!("lcb_remove3() failed");
                //  callback(Err((None, format_error(self.instance, &res))));
            } else if lcb_wait(self.instance) != ErrorType::Success {
                error!("lcb_wait() failed");
                // callback(Err((None, format_error(self.instance, &res))))
            }
        }

        forget(key);

        self
    }

    pub fn remove_sync(&mut self, key: &str) -> OperationResultRemove
    {
        let (tx, rx): (Sender<OperationResultRemove>, Receiver<OperationResultRemove>) = mpsc::channel();
        self.remove(key, move |result: OperationResultRemove| {
            let _ = tx.send(result);
        });

        rx.recv().unwrap()
    }

    /// Will cause the operation to fail _unless_ the key already exists in the cluster.
    pub fn replace<'a, F>(&'a mut self, key: &str, value: &str, cas: u64, exptime: u32, callback: F) -> &Client
        where F: Fn(OperationResultStore) + 'static
    {
        self.store(key, value, Operation::Replace, cas, exptime, callback)
    }

    pub fn replace_sync(&mut self, key: &str, value: &str, cas: u64, exptime: u32) -> OperationResultStore
    {
        self.store_sync(key, value, Operation::Replace, cas, exptime)
    }

    /// Unconditionally store the item in the cluster
    pub fn set<'a, F>(&'a mut self, key: &str, value: &str, cas: u64, exptime: u32, callback: F) -> &Client
        where F: Fn(OperationResultStore) + 'static
    {
        self.store(key, value, Operation::Set, cas, exptime, callback)
    }

    pub fn set_sync(&mut self, key: &str, value: &str, cas: u64, exptime: u32) -> OperationResultStore
    {
        self.store_sync(key, value, Operation::Set, cas, exptime)
    }

    /// Store document in database
    pub fn store<'a, F>(&'a mut self, key: &str, value: &str, operation: Operation, cas: u64, exptime: u32, callback: F) -> &Client
        where F: Fn(OperationResultStore) + 'static
    {
        let key = key.to_owned();

        let mut gcmd = cmd::Store::default();
        gcmd.cas = cas;
        gcmd.exptime = exptime;
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
                    e => {
                        callback(Err((Some(response::Store::new(result)), e)));
                    }
                }
            }));

            let user_data = Box::into_raw(boxed) as *mut Box<Fn(&response::StoreInternal)> as *mut c_void;

            let res = lcb_store3(self.instance, user_data, &gcmd as *const cmd::Store);
            if res != ErrorType::Success {
                error!("lcb_store3() failed");
                // callback(Err((None, format_error(instance, &res))))
            } else if lcb_wait(self.instance) != ErrorType::Success {
                error!("lcb_wait() failed");
                // callback(Err((None, format_error(instance, &res))))
            }
        }

        self
    }

    pub fn store_sync(&mut self, key: &str, value: &str, operation: Operation, cas: u64, exptime: u32) -> OperationResultStore
    {
        let (tx, rx): (Sender<OperationResultStore>, Receiver<OperationResultStore>) = mpsc::channel();
        self.store(key, value, operation, cas, exptime, move |result: OperationResultStore| {
            let _ = tx.send(result);
        });

        rx.recv().unwrap()
    }

    /// Behaviorally it is identical to set in that it will make the server unconditionally store the item, whether it exists or not.
    pub fn upsert<'a, F>(&'a mut self, key: &str, value: &str, cas: u64, exptime: u32, callback: F) -> &Client
        where F: Fn(OperationResultStore) + 'static
    {
        self.store(key, value, Operation::Upsert, cas, exptime, callback)
    }

    pub fn upsert_sync(&mut self, key: &str, value: &str, cas: u64, exptime: u32) -> OperationResultStore
    {
        self.store_sync(key, value, Operation::Upsert, cas, exptime)
    }

    /// Store document in database
    pub fn view_query<'a, F>(&'a mut self, ddoc: &str, view: &str, callback: F) -> &Client
        where F: Fn(OperationResultViewQuery) + 'static
    {
        unsafe {
            extern "C" fn callback_helper(_instance: *mut Instance, _cbtype: CallbackType, raw_row: *const response::ViewQueryInternal) {
                let row = unsafe { &(*raw_row) };
                if row.rflags == 1 {
                    unsafe {
                        let cb: Box<Box<Fn(&response::ViewQueryInternal)>> = Box::from_raw(row.cookie as *mut Box<Fn(&response::ViewQueryInternal)>);
                        (*cb)(row);
                    }
                }
            }

            let mut gcmd = cmd::ViewQuery::default();
            gcmd.cmdflags |= 1 << 16;  // LCB_CMDVIEWQUERY_F_INCLUDE_DOCS;
            gcmd.ddoc = ddoc.as_bytes().as_ptr() as *const libc::c_void;
            gcmd.nddoc = ddoc.len() as u64;
            gcmd.view = view.as_bytes().as_ptr() as *const libc::c_void;
            gcmd.nview = view.len() as u64;
            gcmd.callback = callback_helper as *mut libc::c_void;

            let boxed: OperationResultViewQueryInternalCallback = Box::new(Box::new(move |result: &response::ViewQueryInternal| {
                match result.rc {
                    ErrorType::Success => {
                        debug!("{:?}", result);
                        callback(Ok(response::ViewQuery::new(result)));
                    },
                    e => {
                        callback(Err((Some(response::ViewQuery::new(result)), e)));
                    }
                }
            }));

            let user_data = Box::into_raw(boxed) as *mut Box<Fn(&response::ViewQueryInternal)> as *mut c_void;

            let res = lcb_view_query(self.instance, user_data, &gcmd as *const cmd::ViewQuery);
            if res != ErrorType::Success {
                error!("lcb_view_query() failed");
                // callback(Err((None, format_error(self.instance, &res))))
            } else if lcb_wait(self.instance) != ErrorType::Success {
                error!("lcb_wait() failed")
                // callback(Err((None, format_error(self.instance, &res))))
            }
        }

        self
    }

    pub fn view_query_sync(&mut self, ddoc: &str, view: &str) -> OperationResultViewQuery
    {
        let (tx, rx): (Sender<OperationResultViewQuery>, Receiver<OperationResultViewQuery>) = mpsc::channel();
        self.view_query(ddoc, view, move |result: OperationResultViewQuery| {
            let _ = tx.send(result);
        });

        rx.recv().unwrap()
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        unsafe {
            info!("Disconnecting from {}", &self.uri);
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
            let callback: Box<Box<Fn(&response::GetInternal)>> = Box::from_raw(cookie as *mut Box<Fn(&response::GetInternal)>);
            (*callback)(&(*gresp));
        },
        CallbackType::Remove => {
            debug!("Remove Callback Called!");
            let gresp = resp as *const response::RemoveInternal;
            debug!("{:?}", *gresp);

            let cookie = (*gresp).cookie;
            let callback: Box<Box<Fn(&response::RemoveInternal)>> = Box::from_raw(cookie as *mut Box<Fn(&response::RemoveInternal)>);
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
