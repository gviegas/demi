// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use std::mem;
use std::result;

use crate::{DestroyDevice, Device, InstanceFp};

/// Device-level commands.
pub struct DeviceFp {
    device: Device,

    destroy_device: DestroyDevice,
    // TODO...
}

impl DeviceFp {
    /// Initializes the function pointers for a given `Device`.
    ///
    /// `device` must have been created from `instance_fp`.
    pub unsafe fn new(device: Device, instance_fp: &InstanceFp) -> result::Result<Self, String> {
        debug_assert!(!device.is_null());

        macro_rules! get {
            ($bs:expr) => {
                match instance_fp.get_device_proc_addr(device, $bs.as_ptr().cast()) {
                    Some(x) => Ok(mem::transmute(x)),
                    None => Err(format!(
                        "could not obtain FP: {}",
                        String::from_utf8_lossy(&$bs[..$bs.len() - 1])
                    )),
                }
            };
        }

        Ok(Self {
            device,
            destroy_device: get!(b"vkDestroyDevice\0")?,
        })
    }
}
