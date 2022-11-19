// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use std::ffi::c_char;
use std::mem;
use std::result;

use crate::init::PROC;
use crate::{
    AllocationCallbacks, Bool32, CreateDevice, DestroyInstance, DestroySurfaceKhr, Device,
    DeviceCreateInfo, EnumerateDeviceExtensionProperties, EnumeratePhysicalDeviceGroups,
    EnumeratePhysicalDevices, ExtensionProperties, Format, FormatProperties, GetDeviceProcAddr,
    GetPhysicalDeviceFeatures, GetPhysicalDeviceFeatures2, GetPhysicalDeviceFormatProperties,
    GetPhysicalDeviceMemoryProperties, GetPhysicalDeviceProperties, GetPhysicalDeviceProperties2,
    GetPhysicalDeviceQueueFamilyProperties, GetPhysicalDeviceSurfaceCapabilitiesKhr,
    GetPhysicalDeviceSurfaceFormatsKhr, GetPhysicalDeviceSurfacePresentModesKhr,
    GetPhysicalDeviceSurfaceSupportKhr, Instance, PhysicalDevice, PhysicalDeviceFeatures,
    PhysicalDeviceFeatures2, PhysicalDeviceGroupProperties, PhysicalDeviceMemoryProperties,
    PhysicalDeviceProperties, PhysicalDeviceProperties2, PresentModeKhr, QueueFamilyProperties,
    Result, SurfaceCapabilitiesKhr, SurfaceFormatKhr, SurfaceKhr, VoidFunction,
};

#[cfg(target_os = "linux")]
use crate::{CreateWaylandSurfaceKhr, WaylandSurfaceCreateInfoKhr};

#[cfg(windows)]
use crate::{CreateWin32SurfaceKhr, Win32SurfaceCreateInfoKhr};

#[cfg(all(
    unix,
    not(target_os = "android"),
    not(target_os = "ios"),
    not(target_os = "macos")
))]
use crate::{CreateXcbSurfaceKhr, XcbSurfaceCreateInfoKhr};

/// Instance-level commands.
#[derive(Debug)]
pub struct InstanceFp {
    destroy_instance: DestroyInstance,
    enumerate_physical_devices: EnumeratePhysicalDevices,
    get_physical_device_properties: GetPhysicalDeviceProperties,
    get_physical_device_queue_family_properties: GetPhysicalDeviceQueueFamilyProperties,
    get_physical_device_memory_properties: GetPhysicalDeviceMemoryProperties,
    get_physical_device_features: GetPhysicalDeviceFeatures,
    get_physical_device_format_properties: GetPhysicalDeviceFormatProperties,
    enumerate_device_extension_properties: EnumerateDeviceExtensionProperties,
    create_device: CreateDevice,

    get_device_proc_addr: GetDeviceProcAddr,

    // v1.1
    enumerate_physical_device_groups: Option<EnumeratePhysicalDeviceGroups>,
    get_physical_device_properties_2: Option<GetPhysicalDeviceProperties2>,
    get_physical_device_features_2: Option<GetPhysicalDeviceFeatures2>,

    // VK_KHR_wayland_surface
    #[cfg(target_os = "linux")]
    create_wayland_surface_khr: Option<CreateWaylandSurfaceKhr>,

    // VK_KHR_win32_surface
    #[cfg(windows)]
    create_win32_surface_khr: Option<CreateWin32SurfaceKhr>,

    // VK_KHR_xcb_surface
    #[cfg(all(
        unix,
        not(target_os = "android"),
        not(target_os = "ios"),
        not(target_os = "macos")
    ))]
    create_xcb_surface_khr: Option<CreateXcbSurfaceKhr>,

    // VK_KHR_surface
    destroy_surface_khr: Option<DestroySurfaceKhr>,
    get_physical_device_surface_support_khr: Option<GetPhysicalDeviceSurfaceSupportKhr>,
    get_physical_device_surface_capabilities_khr: Option<GetPhysicalDeviceSurfaceCapabilitiesKhr>,
    get_physical_device_surface_formats_khr: Option<GetPhysicalDeviceSurfaceFormatsKhr>,
    get_physical_device_surface_present_modes_khr: Option<GetPhysicalDeviceSurfacePresentModesKhr>,
}

impl InstanceFp {
    /// Initializes the function pointers for a given `Instance`.
    pub unsafe fn new(instance: Instance) -> result::Result<Self, String> {
        if instance.is_null() {
            return Err(String::from("InstanceFp::new: instance should be non-null"));
        }

        let get = PROC.as_ref().unwrap().fp();

        macro_rules! get {
            ($bs:expr) => {
                match get(instance, $bs.as_ptr().cast()) {
                    Some(x) => Ok(mem::transmute(x)),
                    None => Err(format!(
                        "could not obtain FP: {}",
                        String::from_utf8_lossy(&$bs[..$bs.len() - 1])
                    )),
                }
            };
        }

        Ok(Self {
            destroy_instance: get!(b"vkDestroyInstance\0")?,
            enumerate_physical_devices: get!(b"vkEnumeratePhysicalDevices\0")?,
            get_physical_device_properties: get!(b"vkGetPhysicalDeviceProperties\0")?,
            get_physical_device_queue_family_properties: get!(
                b"vkGetPhysicalDeviceQueueFamilyProperties\0"
            )?,
            get_physical_device_memory_properties: get!(b"vkGetPhysicalDeviceMemoryProperties\0")?,
            get_physical_device_features: get!(b"vkGetPhysicalDeviceFeatures\0")?,
            get_physical_device_format_properties: get!(b"vkGetPhysicalDeviceFormatProperties\0")?,
            enumerate_device_extension_properties: get!(b"vkEnumerateDeviceExtensionProperties\0")?,
            create_device: get!(b"vkCreateDevice\0")?,

            get_device_proc_addr: get!(b"vkGetDeviceProcAddr\0")?,

            enumerate_physical_device_groups: get!(b"vkEnumeratePhysicalDeviceGroups\0").ok(),
            get_physical_device_properties_2: get!(b"vkGetPhysicalDeviceProperties2\0").ok(),
            get_physical_device_features_2: get!(b"vkGetPhysicalDeviceFeatures2\0").ok(),

            #[cfg(target_os = "linux")]
            create_wayland_surface_khr: get!(b"vkCreateWaylandSurfaceKHR\0").ok(),

            #[cfg(windows)]
            create_win32_surface_khr: get!(b"vkCreateWin32SurfaceKHR\0").ok(),

            #[cfg(all(
                unix,
                not(target_os = "android"),
                not(target_os = "ios"),
                not(target_os = "macos")
            ))]
            create_xcb_surface_khr: get!(b"vkCreateXcbSurfaceKHR\0").ok(),

            destroy_surface_khr: get!(b"vkDestroySurfaceKHR\0").ok(),
            get_physical_device_surface_support_khr: get!(
                b"vkGetPhysicalDeviceSurfaceSupportKHR\0"
            )
            .ok(),
            get_physical_device_surface_capabilities_khr: get!(
                b"vkGetPhysicalDeviceSurfaceCapabilitiesKHR\0"
            )
            .ok(),
            get_physical_device_surface_formats_khr: get!(
                b"vkGetPhysicalDeviceSurfaceFormatsKHR\0"
            )
            .ok(),
            get_physical_device_surface_present_modes_khr: get!(
                b"vkGetPhysicalDeviceSurfacePresentModesKHR\0"
            )
            .ok(),
        })
    }

    /// vkGetDeviceProcAddr
    pub(crate) unsafe fn get_device_proc_addr(
        &self,
        device: Device,
        name: *const c_char,
    ) -> Option<VoidFunction> {
        debug_assert!(!device.is_null() && !name.is_null());
        (self.get_device_proc_addr)(device, name)
    }
}

impl InstanceFp {
    /// vkDestroyInstance
    ///
    /// The `InstanceFp` must not be used anymore.
    pub unsafe fn destroy_instance(
        &mut self,
        instance: Instance,
        allocator: *const AllocationCallbacks,
    ) {
        (self.destroy_instance)(instance, allocator);
    }

    /// vkEnumeratePhysicalDevices
    pub unsafe fn enumerate_physical_devices(
        &self,
        instance: Instance,
        physical_device_count: *mut u32,
        physical_devices: *mut PhysicalDevice,
    ) -> Result {
        (self.enumerate_physical_devices)(instance, physical_device_count, physical_devices)
    }

    /// vkEnumeratePhysicalDeviceGroups (v1.1)
    pub unsafe fn enumerate_physical_device_groups(
        &self,
        instance: Instance,
        physical_device_group_count: *mut u32,
        physical_device_group_properties: *mut PhysicalDeviceGroupProperties,
    ) -> Result {
        debug_assert!(self.enumerate_physical_device_groups.is_some());
        (self.enumerate_physical_device_groups.unwrap_unchecked())(
            instance,
            physical_device_group_count,
            physical_device_group_properties,
        )
    }

    /// vkCreateWaylandInstanceKHR (VK_KHR_wayland_surface)
    #[cfg(target_os = "linux")]
    pub unsafe fn create_wayland_surface_khr(
        &self,
        instance: Instance,
        create_info: *const WaylandSurfaceCreateInfoKhr,
        allocator: *const AllocationCallbacks,
        surface: *mut SurfaceKhr,
    ) -> Result {
        debug_assert!(self.create_wayland_surface_khr.is_some());
        (self.create_wayland_surface_khr.unwrap_unchecked())(
            instance,
            create_info,
            allocator,
            surface,
        )
    }

    /// vkCreateWin32SurfaceKHR (VK_KHR_win32_surface)
    #[cfg(windows)]
    pub unsafe fn create_win32_surface_khr(
        &self,
        instance: Instance,
        create_info: *const Win32SurfaceCreateInfoKhr,
        allocator: *const AllocationCallbacks,
        surface: *mut SurfaceKhr,
    ) -> Result {
        debug_assert!(self.create_win32_surface_khr.is_some());
        (self.create_win32_surface_khr.unwrap_unchecked())(
            instance,
            create_info,
            allocator,
            surface,
        )
    }

    /// vkCreateXcbSurfaceKHR (VK_KHR_xcb_surface)
    #[cfg(all(
        unix,
        not(target_os = "android"),
        not(target_os = "ios"),
        not(target_os = "macos")
    ))]
    pub unsafe fn create_xcb_surface_khr(
        &self,
        instance: Instance,
        create_info: *const XcbSurfaceCreateInfoKhr,
        allocator: *const AllocationCallbacks,
        surface: *mut SurfaceKhr,
    ) -> Result {
        debug_assert!(self.create_xcb_surface_khr.is_some());
        (self.create_xcb_surface_khr.unwrap_unchecked())(instance, create_info, allocator, surface)
    }

    /// vkDestroySurfaceKHR (VK_KHR_surface)
    pub unsafe fn destroy_surface_khr(
        &self,
        instance: Instance,
        surface: SurfaceKhr,
        allocator: *const AllocationCallbacks,
    ) {
        debug_assert!(self.destroy_surface_khr.is_some());
        (self.destroy_surface_khr.unwrap_unchecked())(instance, surface, allocator);
    }
}

impl InstanceFp {
    /// vkGetPhysicalDeviceProperties
    pub unsafe fn get_physical_device_properties(
        &self,
        physical_device: PhysicalDevice,
        properties: *mut PhysicalDeviceProperties,
    ) {
        (self.get_physical_device_properties)(physical_device, properties);
    }

    /// vkGetPhysicalDeviceProperties2 (v1.1)
    pub unsafe fn get_physical_device_properties_2(
        &self,
        physical_device: PhysicalDevice,
        properties: *mut PhysicalDeviceProperties2,
    ) {
        debug_assert!(self.get_physical_device_properties_2.is_some());
        (self.get_physical_device_properties_2.unwrap_unchecked())(physical_device, properties);
    }

    /// vkGetPhysicalDeviceQueueFamilyProperties
    pub unsafe fn get_physical_device_queue_family_properties(
        &self,
        physical_device: PhysicalDevice,
        queue_family_property_count: *mut u32,
        queue_family_properties: *mut QueueFamilyProperties,
    ) {
        (self.get_physical_device_queue_family_properties)(
            physical_device,
            queue_family_property_count,
            queue_family_properties,
        );
    }

    /// vkGetPhysicalDeviceMemoryProperties
    pub unsafe fn get_physical_device_memory_properties(
        &self,
        physical_device: PhysicalDevice,
        memory_properties: *mut PhysicalDeviceMemoryProperties,
    ) {
        (self.get_physical_device_memory_properties)(physical_device, memory_properties);
    }

    /// vkGetPhysicalDeviceFeatures
    pub unsafe fn get_physical_device_features(
        &self,
        physical_device: PhysicalDevice,
        features: *mut PhysicalDeviceFeatures,
    ) {
        (self.get_physical_device_features)(physical_device, features);
    }

    /// vkGetPhysicalDeviceFeatures2 (v1.1)
    pub unsafe fn get_physical_device_features_2(
        &self,
        physical_device: PhysicalDevice,
        features: *mut PhysicalDeviceFeatures2,
    ) {
        debug_assert!(self.get_physical_device_features_2.is_some());
        (self.get_physical_device_features_2.unwrap_unchecked())(physical_device, features);
    }

    /// vkGetPhysicalDeviceFormatProperties
    pub unsafe fn get_physical_device_format_properties(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        format_properties: *mut FormatProperties,
    ) {
        (self.get_physical_device_format_properties)(physical_device, format, format_properties);
    }

    /// vkGetPhysicalDeviceSurfaceSupportKHR (VK_KHR_surface)
    pub unsafe fn get_physical_device_surface_support_khr(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        surface: SurfaceKhr,
        supported: *mut Bool32,
    ) -> Result {
        debug_assert!(self.get_physical_device_surface_support_khr.is_some());
        (self
            .get_physical_device_surface_support_khr
            .unwrap_unchecked())(physical_device, queue_family_index, surface, supported)
    }

    /// vkGetPhysicalDeviceSurfaceCapabilitiesKHR (VK_KHR_surface)
    pub unsafe fn get_physical_device_surface_capabilities_khr(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKhr,
        surface_capabilities: *mut SurfaceCapabilitiesKhr,
    ) -> Result {
        debug_assert!(self.get_physical_device_surface_capabilities_khr.is_some());
        (self
            .get_physical_device_surface_capabilities_khr
            .unwrap_unchecked())(physical_device, surface, surface_capabilities)
    }

    /// vkGetPhysicalDeviceSurfaceFormatsKHR (VK_KHR_surface)
    pub unsafe fn get_physical_device_surface_formats_khr(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKhr,
        surface_format_count: *mut u32,
        surface_formats: *mut SurfaceFormatKhr,
    ) -> Result {
        debug_assert!(self.get_physical_device_surface_formats_khr.is_some());
        (self
            .get_physical_device_surface_formats_khr
            .unwrap_unchecked())(
            physical_device,
            surface,
            surface_format_count,
            surface_formats,
        )
    }

    /// vkGetPhysicalDeviceSurfacePresentModesKHR (VK_KHR_surface)
    pub unsafe fn get_physical_device_surface_present_modes_khr(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKhr,
        present_mode_count: *mut u32,
        present_modes: *mut PresentModeKhr,
    ) -> Result {
        debug_assert!(self.get_physical_device_surface_present_modes_khr.is_some());
        (self
            .get_physical_device_surface_present_modes_khr
            .unwrap_unchecked())(physical_device, surface, present_mode_count, present_modes)
    }

    /// vkEnumerateDeviceExtensionProperties
    pub unsafe fn enumerate_device_extension_properties(
        &self,
        physical_device: PhysicalDevice,
        layer_name: *const c_char,
        property_count: *mut u32,
        properties: *mut ExtensionProperties,
    ) -> Result {
        (self.enumerate_device_extension_properties)(
            physical_device,
            layer_name,
            property_count,
            properties,
        )
    }

    /// vkCreateDevice
    pub unsafe fn create_device(
        &self,
        physical_device: PhysicalDevice,
        create_info: *const DeviceCreateInfo,
        allocator: *const AllocationCallbacks,
        device: *mut Device,
    ) -> Result {
        (self.create_device)(physical_device, create_info, allocator, device)
    }
}
