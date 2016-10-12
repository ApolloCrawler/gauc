extern crate clap;
extern crate gauc;
extern crate libc;

use gauc::couchbase::*;

use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;

fn main() {
   let connstr = CString::new("couchbase://localhost/default").unwrap();

    let mut cropts = CreateSt::default();
    cropts.v3.connstr = connstr.as_ptr();

    let mut instance: LcbT = ptr::null_mut();
    unsafe {
        let res = lcb_create(&mut instance as *mut LcbT, &cropts as *const CreateSt);
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

        lcb_install_callback3(instance, CallbackType::Get, Some(op_callback));

        let key = "foo";
        let ckey = CString::new(key).unwrap();
        let mut gcmd = CmdGet::default();
        gcmd.key._type = KvBufferType::Copy;
        gcmd.key.contig.bytes = ckey.as_ptr() as *const libc::c_void;
        gcmd.key.contig.nbytes = key.len() as u64;

        let res = lcb_get3(instance, std::ptr::null(), &gcmd as *const CmdGet);
        println!("Get Res: {:?}", res);

        let res = lcb_wait(instance);
        println!("Get Wait Res: {:?}", res);

        lcb_destroy(instance);
    }
}

unsafe extern "C" fn op_callback(_instance: LcbT, cbtype: CallbackType, resp: *const ResponseBase) {
    match cbtype {
        CallbackType::Get => {
            println!("> Get Callback!");
            let gresp = resp as *const ResponseGet;
            println!(">> CAS: {}", (*gresp).cas);

            if (*gresp).value.is_null() == false {
                let res = CString::from_raw((*gresp).value as *mut i8);
                let length = (*gresp).nvalue as usize;

                println!(">> Content: {}", &res.into_string().unwrap()[..length]);
            }
        },
        _ => panic!("! Unknown Callback...")
    };
}
