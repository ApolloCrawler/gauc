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
