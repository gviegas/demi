//! dlopen API.
//!
//! Do not use it directly; use the [`dl`](../dl/index.html) wrapper instead.

#![cfg(unix)]

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

pub const RTLD_DEFAULT: *const c_char = ptr::null();

pub const RTLD_LAZY: c_int = 0x1;
pub const RTLD_NOW: c_int = 0x2;

pub const RTLD_GLOBAL: c_int = 0x100;
pub const RTLD_LOCAL: c_int = 0;

#[link(name = "dl")]
extern "C" {
    pub fn dlopen(name: *const c_char, flags: c_int) -> *mut c_void;
    pub fn dlclose(handle: *mut c_void) -> c_int;
    pub fn dlsym(handle: *mut c_void, name: *const c_char) -> *mut c_void;
    pub fn dlerror() -> *mut c_char;
}
