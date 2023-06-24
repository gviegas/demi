//! Wrapper exposing basic functionality of the dlopen API.

#![cfg(unix)]

use std::ffi::{c_int, c_void, CStr, CString};

/// Handle to a shared object.
#[derive(Debug)]
pub struct Dl(*mut c_void);

/// Dynamic symbol.
pub type Dlsym = *mut c_void;

impl Dl {
    /// Opens a shared object.
    pub fn new(name: &str, flags: Flags) -> Result<Dl, String> {
        let name = CString::new(name).unwrap();
        let flags = convert_flags(flags);
        unsafe {
            match dl_sys::dlopen(name.as_ptr(), flags) {
                x if x.is_null() => Err(get_error_string()),
                x => Ok(Dl(x)),
            }
        }
    }

    /// Gets a dynamic symbol.
    pub fn get(&self, name: &str) -> Result<Dlsym, String> {
        let name = CString::new(name).unwrap();
        unsafe {
            match dl_sys::dlsym(self.0, name.as_ptr()) {
                x if x.is_null() => Err(get_error_string()),
                x => Ok(x),
            }
        }
    }
}

impl Drop for Dl {
    /// Closes the shared object.
    ///
    /// [`Dlsym`] symbols obtained from `self` must not be used
    /// after dropping.
    fn drop(&mut self) {
        unsafe {
            dl_sys::dlclose(self.0);
        }
    }
}

/// RTLD_* flags.
///
/// The default is [`LAZY`] | [`LOCAL`].
pub type Flags = u32;
pub const LAZY: Flags = 0;
pub const NOW: Flags = 0x1;
pub const LOCAL: Flags = 0;
pub const GLOBAL: Flags = 0x2;

fn convert_flags(flags: Flags) -> c_int {
    let flg = if flags & NOW != 0 {
        dl_sys::RTLD_NOW
    } else {
        dl_sys::RTLD_LAZY
    };
    if flags & GLOBAL != 0 {
        flg | dl_sys::RTLD_GLOBAL
    } else {
        flg | dl_sys::RTLD_LOCAL
    }
}

fn get_error_string() -> String {
    unsafe {
        let err = dl_sys::dlerror();
        assert!(!err.is_null());
        let cstr = CStr::from_ptr(err);
        let cow = String::from_utf8_lossy(cstr.to_bytes());
        cow.to_string()
    }
}
