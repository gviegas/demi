// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use std::mem;

use crate::init::PROC;
use crate::{DestroyInstance, Instance};

/// Instance-level commands.
pub struct InstanceFp {
    instance: Instance,

    destroy_instance: DestroyInstance,
    // TODO...
}

impl InstanceFp {
    /// Initializes the function pointers for a given `Instance`.
    pub fn new(instance: Instance) -> Result<Self, String> {
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
        })
    }
}
