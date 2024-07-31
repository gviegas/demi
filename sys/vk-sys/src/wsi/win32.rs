#![cfg(windows)]

use std::ffi::c_void;

use crate::{AllocationCallbacks, Instance, Result, StructureType, SurfaceKhr};

/// VkWin32SurfaceCreateInfoKHR (VK_KHR_win32_surface)
#[derive(Debug)]
#[repr(C)]
pub struct Win32SurfaceCreateInfoKhr {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: Win32SurfaceCreateFlagsKhr,
    pub h_instance: *mut c_void, // TODO: HINSTANCE
    pub h_wnd: *mut c_void,      // TODO: HWND
}

def_flags!(Win32SurfaceCreateFlagsKhr, Win32SurfaceCreateFlagBitsKhr,);

/// PFN_vkCreateWin32SurfaceKHR (VK_KHR_win32_surface)
pub(crate) type CreateWin32SurfaceKhr = unsafe extern "C" fn(
    instance: Instance,
    info: *const Win32SurfaceCreateInfoKhr,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKhr,
) -> Result;
