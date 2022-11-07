// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use std::mem;
use std::result;

use crate::init::PROC;
use crate::{
    AllocationCallbacks, CreateDevice, DestroyInstance, Device, DeviceCreateInfo,
    EnumeratePhysicalDeviceGroups, EnumeratePhysicalDevices, Format, FormatProperties,
    GetPhysicalDeviceFeatures, GetPhysicalDeviceFeatures2, GetPhysicalDeviceFormatProperties,
    GetPhysicalDeviceMemoryProperties, GetPhysicalDeviceProperties,
    GetPhysicalDeviceQueueFamilyProperties, Instance, PhysicalDevice, PhysicalDeviceFeatures,
    PhysicalDeviceFeatures2, PhysicalDeviceGroupProperties, PhysicalDeviceMemoryProperties,
    PhysicalDeviceProperties, QueueFamilyProperties, Result,
};

/// Instance-level commands.
pub struct InstanceFp {
    instance: Instance,

    destroy_instance: DestroyInstance,
    enumerate_physical_devices: EnumeratePhysicalDevices,
    get_physical_device_properties: GetPhysicalDeviceProperties,
    get_physical_device_queue_family_properties: GetPhysicalDeviceQueueFamilyProperties,
    get_physical_device_memory_properties: GetPhysicalDeviceMemoryProperties,
    get_physical_device_features: GetPhysicalDeviceFeatures,
    get_physical_device_format_properties: GetPhysicalDeviceFormatProperties,
    create_device: CreateDevice,

    // v1.1
    enumerate_physical_device_groups: Option<EnumeratePhysicalDeviceGroups>,
    get_physical_device_features_2: Option<GetPhysicalDeviceFeatures2>,
}

impl InstanceFp {
    /// Initializes the function pointers for a given `Instance`.
    pub fn new(instance: Instance) -> result::Result<Self, String> {
        if instance.is_null() {
            return Err(String::from("InstanceFp::new: instance should be non-null"));
        }

        let get = unsafe { PROC.as_ref().unwrap().fp() };

        macro_rules! get {
            ($bs:expr) => {
                unsafe {
                    match get(instance, $bs.as_ptr().cast()) {
                        Some(x) => Ok(mem::transmute(x)),
                        None => Err(format!(
                            "could not obtain FP: {}",
                            String::from_utf8_lossy(&$bs[..$bs.len() - 1])
                        )),
                    }
                }
            };
        }

        Ok(Self {
            instance,
            destroy_instance: get!(b"vkDestroyInstance\0")?,
            enumerate_physical_devices: get!(b"vkEnumeratePhysicalDevices\0")?,
            get_physical_device_properties: get!(b"vkGetPhysicalDeviceProperties\0")?,
            get_physical_device_queue_family_properties: get!(
                b"vkGetPhysicalDeviceQueueFamilyProperties\0"
            )?,
            get_physical_device_memory_properties: get!(b"vkGetPhysicalDeviceMemoryProperties\0")?,
            get_physical_device_features: get!(b"vkGetPhysicalDeviceFeatures\0")?,
            get_physical_device_format_properties: get!(b"vkGetPhysicalDeviceFormatProperties\0")?,
            create_device: get!(b"vkCreateDevice\0")?,
            enumerate_physical_device_groups: get!(b"vkEnumeratePhysicalDeviceGroups\0").ok(),
            get_physical_device_features_2: get!(b"vkGetPhysicalDeviceFeatures2\0").ok(),
        })
    }
}

impl InstanceFp {
    /// vkEnumeratePhysicalDevices
    pub unsafe fn enumerate_physical_devices(
        &self,
        physical_device_count: *mut u32,
        physical_devices: *mut PhysicalDevice,
    ) -> Result {
        (self.enumerate_physical_devices)(self.instance, physical_device_count, physical_devices)
    }

    /// vkEnumeratePhysicalDeviceGroups
    /// [v1.1]
    pub unsafe fn enumerate_physical_device_groups(
        &self,
        physical_device_group_count: *mut u32,
        physical_device_group_properties: *mut PhysicalDeviceGroupProperties,
    ) -> Result {
        (self.enumerate_physical_device_groups.unwrap())(
            self.instance,
            physical_device_group_count,
            physical_device_group_properties,
        )
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

    /// vkGetPhysicalDeviceFeatures2
    /// [v1.1]
    pub unsafe fn get_physical_device_features_2(
        &self,
        physical_device: PhysicalDevice,
        features: *mut PhysicalDeviceFeatures2,
    ) {
        (self.get_physical_device_features_2.unwrap())(physical_device, features);
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
