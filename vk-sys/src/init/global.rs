// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use crate::init::GLOBAL_FP;
use crate::{AllocationCallbacks, Instance, InstanceCreateInfo, Result, API_VERSION_1_0, SUCCESS};

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
