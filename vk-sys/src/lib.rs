// Copyright 2022 Gustavo C. Viegas. All rights reserved.

#[cfg(unix)]
mod unix {} // TODO

#[cfg(windows)]
mod windows {} // TODO

// Defines a dispatchable handle.
macro_rules! def_dh {
    ($opq:ident, $hdl:ident) => {
        #[repr(C)]
        pub struct $opq {
            _opaque: [u8; 0],
        }
        pub type $hdl = *mut $opq;
    };
}

// Defines a non-dispatchable handle.
// TODO: Pointer instead in 64-bit archs.
macro_rules! def_ndh {
    ($opq:ident, $hdl:ident) => {
        pub type $hdl = u64;
    };
}

// Defines 32-bit flags an their associated types.
macro_rules! def_flags {
    ($mask:ident, $bit:ident, $( $cons:ident = $val:expr ),*) => {
        pub type $mask = u32;
        pub type $bit = u32;
        $( pub const $cons: $bit = $val; )*
    };
}

// Defines 64-bit flags and their associated types.
macro_rules! def_flags64 {
    ($mask:ident, $bit:ident, $( $cons:ident = $val:expr ),*) => {
        pub type $mask = u64;
        pub type $bit = u64;
        $( pub const $cons: $bit = $val; )*
    };
}

// Defines IDs (non-flags) and their associated type.
macro_rules! def_ids {
    ($name:ident, $( $cons:ident = $val:expr ),*) => {
        pub type $name = core::ffi::c_int;
        $( pub const $cons: $name = $val; )*
    };
}

mod core;
pub use crate::core::*;

/// VK_NULL_HANDLE
// TODO: Pointer instead in 64-bit archs.
#[inline]
pub fn null_handle() -> u64 {
    0
}

/// Checks whether `ndh` is equal to `null_handle()`.
// TODO: Pointer instead in 64-bit archs.
#[inline]
pub fn is_null_handle(ndh: u64) -> bool {
    ndh == 0
}
