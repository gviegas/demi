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
        pub type $name = ::core::ffi::c_int;
        $( pub const $cons: $name = $val; )*
    };
}

mod core;
pub use crate::core::*;

mod result;
pub use crate::result::*;

mod stype;
pub use crate::stype::*;

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

/// VkBool32
pub type Bool32 = u32;
pub const TRUE: Bool32 = 1;
pub const FALSE: Bool32 = 0;

/// VkOffset2D
#[repr(C)]
pub struct Offset2d {
    pub x: i32,
    pub y: i32,
}

/// VkOffset3D
#[repr(C)]
pub struct Offset3d {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

/// VkExtent2D
#[repr(C)]
pub struct Extent2d {
    pub width: u32,
    pub height: u32,
}

/// VkExtent3D
#[repr(C)]
pub struct Extent3d {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

/// VkRect2D
#[repr(C)]
pub struct Rect2d {
    pub offset: Offset2d,
    pub extent: Extent2d,
}
