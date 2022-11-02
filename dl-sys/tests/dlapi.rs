// Copyright 2022 Gustavo C. Viegas. All rights reserved.

#![cfg(unix)]

use core::ffi::{c_char, CStr};
use dl_sys;

// XXX: This is unlikely to test much in a non-linux/gnu OS.
#[test]
fn test_api() {
    unsafe {
        let lib = b"libpthread.so.0\0" as *const u8 as *const c_char;
        let handle = dl_sys::dlopen(lib, dl_sys::RTLD_LAZY | dl_sys::RTLD_GLOBAL);
        if handle.is_null() {
            let err = dl_sys::dlerror();
            assert!(!err.is_null());
            println!("{:?}", CStr::from_ptr(err));
            return;
        }

        let syms = [
            b"pthread_create\0" as *const u8 as *const c_char,
            b"pthread_self\0" as *const u8 as *const c_char,
            b"pthread_join\0" as *const u8 as *const c_char,
            b"pthread_self\0" as *const u8 as *const c_char,
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
