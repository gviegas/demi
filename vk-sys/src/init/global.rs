// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use crate::init::FP_GLOBAL;
use crate::{AllocationCallbacks, Instance, InstanceCreateInfo, Result};

/// vkEnumerateInstanceVersion
pub unsafe fn enumerate_instance_version(api_version: *mut u32) -> Result {
    // BUG: Return version 1.0 if this command is not available.
    debug_assert!(FP_GLOBAL
        .as_ref()
        .unwrap()
        .enumerate_instance_version
        .is_some());
    (FP_GLOBAL
        .as_ref()
        .unwrap_unchecked()
        .enumerate_instance_version
        .unwrap_unchecked())(api_version)
}

/// vkCreateInstance
pub unsafe fn create_instance(
    create_info: *const InstanceCreateInfo,
    allocator: *const AllocationCallbacks,
    instance: *mut Instance,
) -> Result {
    debug_assert!(FP_GLOBAL.is_some());
    (FP_GLOBAL.as_ref().unwrap_unchecked().create_instance)(create_info, allocator, instance)
}
