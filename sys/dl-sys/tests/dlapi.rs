#![cfg(unix)]

use core::ffi::CStr;
use dl_sys;

// XXX: This is unlikely to test much in a non-linux/gnu OS.
#[test]
fn test_api() {
    unsafe {
        let lib = c"libpthread.so.0".as_ptr();
        let handle = dl_sys::dlopen(lib, dl_sys::RTLD_LAZY | dl_sys::RTLD_GLOBAL);
        if handle.is_null() {
            let err = dl_sys::dlerror();
            assert!(!err.is_null());
            println!("{:?}", CStr::from_ptr(err));
            return;
        }

        let syms = [
            c"pthread_create".as_ptr(),
            c"pthread_self".as_ptr(),
            c"pthread_join".as_ptr(),
            c"pthread_self".as_ptr(),
        ];
        for i in syms {
            let sym = dl_sys::dlsym(handle, i);
            if sym.is_null() {
                let err = dl_sys::dlerror();
                assert!(!err.is_null());
                assert!(false, "{:?}", CStr::from_ptr(err));
            }
        }

        match dl_sys::dlclose(handle) {
            0 => (),
            _ => {
                let err = dl_sys::dlerror();
                assert!(!err.is_null());
                println!("{:?}", CStr::from_ptr(err));
            }
        }
    }
}
