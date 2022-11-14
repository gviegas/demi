// Copyright 2022 Gustavo C. Viegas. All rights reserved.

#![cfg(target_os = "linux")]

use std::ffi::c_void;

use crate::{AllocationCallbacks, Instance, Result, StructureType, SurfaceKhr};

/// VkWaylandSurfaceCreateInfoKHR (VK_KHR_wayland_surface)
#[derive(Debug)]
#[repr(C)]
pub struct WaylandSurfaceCreateInfoKhr {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: WaylandSurfaceCreateFlagsKhr,
    pub display: *mut wlc_sys::Display,
    pub surface: *mut wlc_sys::Surface,
}

def_flags!(
    WaylandSurfaceCreateFlagsKhr,
    WaylandSurfaceCreateFlagBitsKhr,
);

/// PFN_vkCreateWaylandSurfaceKHR (VK_KHR_wayland_surface)
pub(crate) type CreateWaylandSurfaceKhr = unsafe extern "C" fn(
    instance: Instance,
    info: *const WaylandSurfaceCreateInfoKhr,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKhr,
) -> Result;
