// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use std::ffi::c_void;

#[allow(non_camel_case_types)]
pub type c_size_t = usize; // XXX

mod init;
pub use crate::init::*;

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
#[cfg(not(target_pointer_width = "64"))]
macro_rules! def_ndh {
    ($opq:ident, $hdl:ident) => {
        pub type $hdl = u64;
    };
}
#[cfg(target_pointer_width = "64")]
macro_rules! def_ndh {
    ($opq:ident, $hdl:ident) => {
        def_dh!($opq, $hdl);
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

mod format;
pub use crate::format::*;

mod limits;
pub use crate::limits::*;

mod features;
pub use crate::features::*;

mod wsi;
pub use crate::wsi::*;

/// VK_NULL_HANDLE
#[cfg(not(target_pointer_width = "64"))]
pub const fn null_handle() -> u64 {
    0
}
/// VK_NULL_HANDLE
#[cfg(target_pointer_width = "64")]
pub const fn null_handle<T>() -> *mut T {
    std::ptr::null_mut()
}

/// Checks whether `ndh` is equal to `null_handle()`.
#[cfg(not(target_pointer_width = "64"))]
pub const fn is_null_handle(ndh: u64) -> bool {
    ndh == 0
}
/// Checks whether `ndh` is equal to `null_handle()`.
#[cfg(target_pointer_width = "64")]
pub fn is_null_handle<T>(ndh: *mut T) -> bool {
    ndh.is_null()
}

/// VkBool32
pub type Bool32 = u32;

/// VK_TRUE
pub const TRUE: Bool32 = 1;

/// VK_FALSE
pub const FALSE: Bool32 = 0;

/// VkOffset2D
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Offset2d {
    pub x: i32,
    pub y: i32,
}

/// VkOffset3D
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Offset3d {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

/// VkExtent2D
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Extent2d {
    pub width: u32,
    pub height: u32,
}

/// VkExtent3D
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Extent3d {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

/// VkRect2D
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Rect2d {
    pub offset: Offset2d,
    pub extent: Extent2d,
}

/// VkAllocationCallbacks
#[derive(Debug)]
#[repr(C)]
pub struct AllocationCallbacks {
    pub user_data: *mut c_void,
    pub allocation: AllocationFunction,
    pub reallocation: ReallocationFunction,
    pub free: FreeFunction,
    pub internal_allocation: Option<InternalAllocationNotification>,
    pub internal_free: Option<InternalFreeNotification>,
}

/// PFN_vkAllocationFunction
pub type AllocationFunction = unsafe extern "C" fn(
    user_data: *mut c_void,
    size: c_size_t,
    alignment: c_size_t,
    allocation_scope: SystemAllocationScope,
) -> *mut c_void;

/// PFN_vkReallocationFunction
pub type ReallocationFunction = unsafe extern "C" fn(
    user_data: *mut c_void,
    original: *mut c_void,
    size: c_size_t,
    alignment: c_size_t,
    allocation_scope: SystemAllocationScope,
) -> *mut c_void;

/// PFN_vkFreeFunction
pub type FreeFunction = unsafe extern "C" fn(user_data: *mut c_void, memory: *mut c_void);

/// PFN_vkInternalAllocationNotification
pub type InternalAllocationNotification = unsafe extern "C" fn(
    user_data: *mut c_void,
    size: c_size_t,
    allocation_type: InternalAllocationType,
    allocation_scope: SystemAllocationScope,
);

/// PFN_vkInternalFreeNotification
pub type InternalFreeNotification = InternalAllocationNotification;

def_ids!(
    SystemAllocationScope,
    SYSTEM_ALLOCATION_SCOPE_COMMAND = 0,
    SYSTEM_ALLOCATION_SCOPE_OBJECT = 1,
    SYSTEM_ALLOCATION_SCOPE_CACHE = 2,
    SYSTEM_ALLOCATION_SCOPE_DEVICE = 3,
    SYSTEM_ALLOCATION_SCOPE_INSTANCE = 4
);

def_ids!(
    InternalAllocationType,
    INTERNAL_ALLOCATION_TYPE_EXECUTABLE = 0
);

/// VK_API_VERSION_VARIANT
pub const fn api_version_variant(version: u32) -> u32 {
    version >> 29
}

/// VK_API_VERSION_MAJOR
pub const fn api_version_major(version: u32) -> u32 {
    version >> 22 & 0x00_00_00_7F
}

/// VK_API_VERSION_MINOR
pub const fn api_version_minor(version: u32) -> u32 {
    version >> 12 & 0x00_00_03_FF
}

/// VK_API_VERSION_PATCH
pub const fn api_version_patch(version: u32) -> u32 {
    version & 0x00_00_0F_FF
}

/// VK_MAKE_API_VERSION
pub const fn make_api_version(variant: u32, major: u32, minor: u32, patch: u32) -> u32 {
    variant << 29 | major << 22 | minor << 12 | patch
}

/// VK_API_VERSION_1_0
pub const API_VERSION_1_0: u32 = make_api_version(0, 1, 0, 0);

/// VK_API_VERSION_1_1
pub const API_VERSION_1_1: u32 = make_api_version(0, 1, 1, 0);

/// VK_API_VERSION_1_2
pub const API_VERSION_1_2: u32 = make_api_version(0, 1, 2, 0);

/// VK_API_VERSION_1_3
pub const API_VERSION_1_3: u32 = make_api_version(0, 1, 3, 0);

/// VK_ATTACHMENT_UNUSED
pub const ATTACHMENT_UNUSED: u32 = u32::MAX;

/// VK_LOD_CLAMP_NONE
pub const LOD_CLAMP_NONE: f32 = 1000.0;

/// VK_QUEUE_FAMILY_IGNORED
pub const QUEUE_FAMILY_IGNORED: u32 = u32::MAX;

/// VK_REMAINING_ARRAY_LAYERS
pub const REMAINING_ARRAY_LAYERS: u32 = u32::MAX;

/// VK_REMAINING_MIP_LEVELS
pub const REMAINING_MIP_LEVELS: u32 = u32::MAX;

/// VK_SUBPASS_EXTERNAL
pub const SUBPASS_EXTERNAL: u32 = u32::MAX;

/// VK_WHOLE_SIZE
pub const WHOLE_SIZE: u64 = u64::MAX;
