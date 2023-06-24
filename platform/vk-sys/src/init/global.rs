use std::ffi::c_char;

use crate::init::GLOBAL_FP;
use crate::{
    AllocationCallbacks, ExtensionProperties, Instance, InstanceCreateInfo, LayerProperties,
    Result, API_VERSION_1_0, SUCCESS,
};

/// vkEnumerateInstanceLayerProperties
pub unsafe fn enumerate_instance_layer_properties(
    property_count: *mut u32,
    properties: *mut LayerProperties,
) -> Result {
    debug_assert!(GLOBAL_FP.is_some());
    (GLOBAL_FP
        .as_ref()
        .unwrap_unchecked()
        .enumerate_instance_layer_properties)(property_count, properties)
}

/// vkEnumerateInstanceExtensionProperties
pub unsafe fn enumerate_instance_extension_properties(
    layer_name: *const c_char,
    property_count: *mut u32,
    properties: *mut ExtensionProperties,
) -> Result {
    debug_assert!(GLOBAL_FP.is_some());
    (GLOBAL_FP
        .as_ref()
        .unwrap_unchecked()
        .enumerate_instance_extension_properties)(layer_name, property_count, properties)
}

/// vkEnumerateInstanceVersion (v1.1)
pub unsafe fn enumerate_instance_version(api_version: *mut u32) -> Result {
    debug_assert!(GLOBAL_FP.is_some());
    if let Some(fp) = GLOBAL_FP
        .as_ref()
        .unwrap_unchecked()
        .enumerate_instance_version
    {
        fp(api_version)
    } else {
        debug_assert!(!api_version.is_null());
        *api_version = API_VERSION_1_0;
        SUCCESS
    }
}

/// vkCreateInstance
pub unsafe fn create_instance(
    create_info: *const InstanceCreateInfo,
    allocator: *const AllocationCallbacks,
    instance: *mut Instance,
) -> Result {
    debug_assert!(GLOBAL_FP.is_some());
    (GLOBAL_FP.as_ref().unwrap_unchecked().create_instance)(create_info, allocator, instance)
}
