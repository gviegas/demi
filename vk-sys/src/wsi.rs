// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use std::ffi::c_void;

use crate::{
    AllocationCallbacks, Bool32, Device, Extent2d, Fence, Format, Image, ImageUsageFlags, Instance,
    PhysicalDevice, Queue, Result, Semaphore, SharingMode, StructureType,
};

def_ndh!(SurfaceKhrT, SurfaceKhr);

mod wayland;
#[cfg(target_os = "linux")]
pub use crate::wsi::wayland::*;

mod win32;
#[cfg(windows)]
pub use crate::wsi::win32::*;

mod xcb;
#[cfg(all(
    unix,
    not(target_os = "android"),
    not(target_os = "ios"),
    not(target_os = "macos")
))]
pub use crate::wsi::xcb::*;

/// VkSurfaceCapabilitiesKHR (VK_KHR_surface)
#[derive(Debug)]
#[repr(C)]
pub struct SurfaceCapabilitiesKhr {
    pub min_image_count: u32,
    pub max_image_count: u32,
    pub current_extent: Extent2d,
    pub min_image_extent: Extent2d,
    pub max_image_extent: Extent2d,
    pub max_image_array_layers: u32,
    pub supported_transforms: SurfaceTransformFlagsKhr,
    pub current_transform: SurfaceTransformFlagBitsKhr,
    pub supported_composite_alpha: CompositeAlphaFlagsKhr,
    pub supported_usage_flags: ImageUsageFlags,
}

def_flags!(
    SurfaceTransformFlagsKhr,
    SurfaceTransformFlagBitsKhr,
    SURFACE_TRANSFORM_IDENTITY_BIT_KHR = 0x00000001,
    SURFACE_TRANSFORM_ROTATE_90_BIT_KHR = 0x00000002,
    SURFACE_TRANSFORM_ROTATE_180_BIT_KHR = 0x00000004,
    SURFACE_TRANSFORM_ROTATE_270_BIT_KHR = 0x00000008,
    SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR = 0x00000010,
    SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR = 0x00000020,
    SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR = 0x00000040,
    SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR = 0x00000080,
    SURFACE_TRANSFORM_INHERIT_BIT_KHR = 0x00000100
);

def_flags!(
    CompositeAlphaFlagsKhr,
    CompositeAlphaFlagBitsKhr,
    COMPOSITE_ALPHA_OPAQUE_BIT_KHR = 0x00000001,
    COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR = 0x00000002,
    COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR = 0x00000004,
    COMPOSITE_ALPHA_INHERIT_BIT_KHR = 0x00000008
);

/// VkSurfaceFormatKHR (VK_KHR_surface)
#[derive(Debug)]
#[repr(C)]
pub struct SurfaceFormatKhr {
    pub format: Format,
    pub color_space: ColorSpaceKhr,
}

def_ids!(
    ColorSpaceKhr,
    COLOR_SPACE_SRGB_NONLINEAR_KHR = 0,
    COLOR_SPACE_DISPLAY_P3_NONLINEAR_EXT = 1000104001,
    COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT = 1000104002,
    COLOR_SPACE_DISPLAY_P3_LINEAR_EXT = 1000104003,
    COLOR_SPACE_DCI_P3_NONLINEAR_EXT = 1000104004,
    COLOR_SPACE_BT709_LINEAR_EXT = 1000104005,
    COLOR_SPACE_BT709_NONLINEAR_EXT = 1000104006,
    COLOR_SPACE_BT2020_LINEAR_EXT = 1000104007,
    COLOR_SPACE_HDR10_ST2084_EXT = 1000104008,
    COLOR_SPACE_DOLBYVISION_EXT = 1000104009,
    COLOR_SPACE_HDR10_HLG_EXT = 1000104010,
    COLOR_SPACE_ADOBERGB_LINEAR_EXT = 1000104011,
    COLOR_SPACE_ADOBERGB_NONLINEAR_EXT = 1000104012,
    COLOR_SPACE_PASS_THROUGH_EXT = 1000104013,
    COLOR_SPACE_EXTENDED_SRGB_NONLINEAR_EXT = 1000104014,
    COLOR_SPACE_DISPLAY_NATIVE_AMD = 1000213000,
    COLORSPACE_SRGB_NONLINEAR_KHR = COLOR_SPACE_SRGB_NONLINEAR_KHR,
    COLOR_SPACE_DCI_P3_LINEAR_EXT = COLOR_SPACE_DISPLAY_P3_LINEAR_EXT
);

def_ids!(
    PresentModeKhr,
    PRESENT_MODE_IMMEDIATE_KHR = 0,
    PRESENT_MODE_MAILBOX_KHR = 1,
    PRESENT_MODE_FIFO_KHR = 2,
    PRESENT_MODE_FIFO_RELAXED_KHR = 3,
    PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR = 1000111000,
    PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR = 1000111001
);

/// PFN_vkDestroySurfaceKHR (VK_KHR_surface)
pub(crate) type DestroySurfaceKhr = unsafe extern "C" fn(
    instance: Instance,
    surface: SurfaceKhr,
    allocator: *const AllocationCallbacks,
);

/// PFN_vkGetPhysicalDeviceSurfaceSupportKHR (VK_KHR_surface)
pub(crate) type GetPhysicalDeviceSurfaceSupportKhr = unsafe extern "C" fn(
    phys_dev: PhysicalDevice,
    fam_idx: u32,
    surface: SurfaceKhr,
    supported: *mut Bool32,
) -> Result;

/// PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR (VK_KHR_surface)
pub(crate) type GetPhysicalDeviceSurfaceCapabilitiesKhr = unsafe extern "C" fn(
    phys_dev: PhysicalDevice,
    surface: SurfaceKhr,
    capabilities: *mut SurfaceCapabilitiesKhr,
) -> Result;

/// PFN_vkGetPhysicalDeviceSurfaceFormatsKHR (VK_KHR_surface)
pub(crate) type GetPhysicalDeviceSurfaceFormatsKhr = unsafe extern "C" fn(
    phys_dev: PhysicalDevice,
    surface: SurfaceKhr,
    count: *mut u32,
    formats: *mut SurfaceFormatKhr,
) -> Result;

/// PFN_vkGetPhysicalDeviceSurfacePresentModesKHR (VK_KHR_surface)
pub(crate) type GetPhysicalDeviceSurfacePresentModesKhr = unsafe extern "C" fn(
    phys_dev: PhysicalDevice,
    surface: SurfaceKhr,
    count: *mut u32,
    present_modes: *mut PresentModeKhr,
) -> Result;

def_ndh!(SwapchainKhrT, SwapchainKhr);

/// VkSwapchainCreateInfoKHR (VK_KHR_swapchain)
#[derive(Debug)]
#[repr(C)]
pub struct SwapchainCreateInfoKhr {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: SwapchainCreateFlagsKhr,
    pub surface: SurfaceKhr,
    pub min_image_count: u32,
    pub image_format: Format,
    pub image_color_space: ColorSpaceKhr,
    pub image_extent: Extent2d,
    pub image_array_layers: u32,
    pub image_usage: ImageUsageFlags,
    pub image_sharing_mode: SharingMode,
    pub queue_family_index_count: u32,
    pub queue_family_indices: *const u32,
    pub pre_transform: SurfaceTransformFlagsKhr,
    pub composite_alpha: CompositeAlphaFlagsKhr,
    pub present_mode: PresentModeKhr,
    pub clipped: Bool32,
    pub old_swapchain: SwapchainKhr,
}

def_flags!(
    SwapchainCreateFlagsKhr,
    SwapchainCreateFlagBitsKhr,
    SWAPCHAIN_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR = 0x00000001,
    SWAPCHAIN_CREATE_PROTECTED_BIT_KHR = 0x00000002,
    SWAPCHAIN_CREATE_MUTABLE_FORMAT_BIT_KHR = 0x00000004
);

/// PFN_vkCreateSwapchainKHR (VK_KHR_swapchain)
pub(crate) type CreateSwapchainKhr = unsafe extern "C" fn(
    device: Device,
    info: *const SwapchainCreateInfoKhr,
    allocator: *const AllocationCallbacks,
    swapchain: *mut SwapchainKhr,
) -> Result;

/// PFN_vkDestroySwapchainKHR (VK_KHR_swapchain)
pub(crate) type DestroySwapchainKhr = unsafe extern "C" fn(
    device: Device,
    swapchain: SwapchainKhr,
    allocator: *const AllocationCallbacks,
) -> Result;

/// PFN_vkGetSwapchainImagesKHR (VK_KHR_swapchain)
pub(crate) type GetSwapchainImagesKhr = unsafe extern "C" fn(
    device: Device,
    swapchain: SwapchainKhr,
    count: *mut u32,
    images: *mut Image,
) -> Result;

/// PFN_vkAcquireNextImageKHR (VK_KHR_swapchain)
pub(crate) type AcquireNextImageKhr = unsafe extern "C" fn(
    device: Device,
    swapchain: SwapchainKhr,
    timeout: u64,
    semaphore: Semaphore,
    fence: Fence,
    index: *mut u32,
) -> Result;

/// VkPresentInfoKHR (VK_KHR_swapchain)
#[derive(Debug)]
#[repr(C)]
pub struct PresentInfoKhr {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub wait_semaphore_count: u32,
    pub wait_semaphores: *const Semaphore,
    pub swapchain_count: u32,
    pub swapchains: *const SwapchainKhr,
    pub image_indices: *const u32,
    pub results: *mut Result,
}

/// PFN_vkQueuePresentKHR (VK_KHR_swapchain)
pub(crate) type QueuePresentKhr =
    unsafe extern "C" fn(queue: Queue, info: *const PresentInfoKhr) -> Result;
