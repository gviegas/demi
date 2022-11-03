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

/// VK_NULL_HANDLE
/// TODO: Pointer instead in 64-bit archs.
#[inline]
pub fn null_handle() -> u64 {
    0
}

/// Checks whether `ndh` is equal to `null_handle()`.
/// TODO: Pointer instead in 64-bit archs.
#[inline]
pub fn is_null_handle(ndh: u64) -> bool {
    ndh == 0
}
