#![cfg(all(
    unix,
    not(target_os = "android"),
    not(target_os = "ios"),
    not(target_os = "macos")
))]

use std::ffi::c_void;

use crate::{AllocationCallbacks, Instance, Result, StructureType, SurfaceKhr};

/// VkXcbSurfaceCreateInfoKHR (VK_KHR_xcb_surface)
#[derive(Debug)]
#[repr(C)]
pub struct XcbSurfaceCreateInfoKhr {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: XcbSurfaceCreateFlagsKhr,
    pub xcb_connection: *mut c_void, // TODO: xcb_connection_t *
    pub xcb_window: u32,             // TODO: xcb_window_t
}

def_flags!(XcbSurfaceCreateFlagsKhr, XcbSurfaceCreateFlagBitsKhr,);

/// PFN_vkCreateXcbSurfaceKHR (VK_KHR_xcb_surface)
pub(crate) type CreateXcbSurfaceKhr = unsafe extern "C" fn(
    instance: Instance,
    info: *const XcbSurfaceCreateInfoKhr,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKhr,
) -> Result;
