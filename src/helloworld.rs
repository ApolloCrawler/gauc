extern crate gauc;
extern crate libc;

use gauc::client::*;
use gauc::couchbase::*;

use std::ffi::CString;
use std::ptr;
use std::ffi::CStr;

fn main() {
    /* let client = */ Client::new("couchbase://localhost/default");

    let connstr = CString::new("couchbase://localhost/default").unwrap();

    let mut cropts = LcbCreateSt::default();
    cropts.v3.connstr = connstr.as_ptr();

    let mut instance: LcbT = ptr::null_mut();
    unsafe {
        let res = lcb_create(&mut instance as *mut LcbT, &cropts as *const LcbCreateSt);
        println!("Create Res: {:?}", res);

        let res = lcb_connect(instance);
        println!("Connect Res: {:?}", res);

        let res = lcb_wait(instance);
        println!("Connect Wait Res: {:?}", res);

        let res = lcb_get_bootstrap_status(instance);
        println!(
            "Bootstrap Status: {:?} \"{}\"",
            res,
            CStr::from_ptr(lcb_strerror(instance, res)).to_str().unwrap() // description
        );

        lcb_install_callback3(instance, LcbCallbackType::LcbCallbackGet ,Some(op_callback));

        let key = "foo";
        let ckey = CString::new(key).unwrap();
        let mut gcmd = LcbCmdGet::default();
        gcmd.key._type = LcbKvBufType::LcbKvCopy;
        gcmd.key.contig.bytes = ckey.as_ptr() as *const libc::c_void;
        gcmd.key.contig.nbytes = key.len() as u64;

        let res = lcb_get3(instance, std::ptr::null(), &gcmd as *const LcbCmdGet);
        println!("Get Res: {:?}", res);

        let res = lcb_wait(instance);
        println!("Get Wait Res: {:?}", res);

        lcb_destroy(instance);
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
