use std::sync::mpsc::{Sender, Receiver};
use std::thread;

// use libc::{c_void};
use std::ffi::CStr;
use std::ffi::CString;
use std::{fmt, process, ptr};
use std::sync::mpsc;

use std::sync::{Arc, Mutex};

use super::super::couchbase::*;

/// Type of libcouchbase Request
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RequestType {
    Quit,
    Get,
    Store
}

/// Request instance
#[derive(Debug, Clone, Copy)]
pub struct Request {
    pub typ: RequestType,
}

/// Implementation of libcouchbase Request methods
impl Request {
    pub fn new(typ: RequestType) -> Request {
        Request {
            typ: typ
        }
    }
}

#[derive(Debug)]
pub struct Proxy {
    pub tx: Arc<Mutex<Sender<Request>>>,
}

impl Proxy {
    pub fn new(tx: Arc<Mutex<Sender<Request>>>) -> Proxy {
        // let connstr = CString::new(uri).unwrap();

        Proxy {
            tx: tx,
        }
    }

    pub fn get(&self, docid: &str) -> String {
        let _res = self.tx.lock().unwrap().send(Request::new(RequestType::Get));
        return docid.to_string()
    }

    pub fn quit(&self) {
        let _ = self.tx.lock().unwrap().send(Request::new(RequestType::Quit));
    }
}

pub struct Worker {
    pub rx: Arc<Mutex<Receiver<Request>>>,
    pub uri: String,
    pub handle: u64,
    pub thread: thread::JoinHandle<()>,
    pub instance: Instance,
}

impl fmt::Debug for Worker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Worker {{ rx: {:?}, uri: {:?} }}", self.rx, self.uri)
    }
}

impl Worker {
    pub fn new(rx: Arc<Mutex<Receiver<Request>>>,uri: &str) -> Worker {
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

            let clonned_rx = rx.clone();
            let res = Worker {
                rx: rx,
                uri: uri.to_string(),
                handle: 0,
                thread: thread::spawn(move || {
                    debug!("Spawned worker thread");
                    let mut running = true;
                    while running {
                        match clonned_rx.lock() {
                            Ok(my_rx) => {
                                let msg = my_rx.recv();
                                debug!("Received worker message {:?}", msg);
                                running = msg.unwrap().typ != RequestType::Quit;
                            }
                            Err(_) => {}
                        }
                    }
                }),
                instance: instance
            };

            return res;
        }
    }

    pub fn get() {}
}

impl Drop for Worker {
    fn drop(&mut self) {
        // self.thread.terminate();
    }
}

#[derive(Debug)]
pub struct Client2 {
    tx: Arc<Mutex<Sender<Request>>>,
    rx: Arc<Mutex<Receiver<Request>>>
}

impl Client2 {
    pub fn new() -> Client2 {
        debug!("Creating Client2");

        let (tx, rx): (Sender<Request>, Receiver<Request>) = mpsc::channel();
        let safe_rx = Arc::new(Mutex::new(rx));
        let safe_tx = Arc::new(Mutex::new(tx));

        Client2 {
            tx: safe_tx,
            rx: safe_rx
        }
    }
}

impl super::CouchbaseClient for Client2 {
    fn get(&self, _docid: &str) {
    }

    fn store(&self, _docid: &str, _value: &str) {
    }
}

impl Drop for Client2 {
    fn drop(&mut self) {
        debug!("Dropping Client2")
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
