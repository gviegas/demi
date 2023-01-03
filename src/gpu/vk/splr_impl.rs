// Copyright 2023 Gustavo C. Viegas. All rights reserved.

use std::io;
use std::ptr;

use vk_sys::{
    Sampler, SamplerCreateInfo, ERROR_OUT_OF_DEVICE_MEMORY, ERROR_OUT_OF_HOST_MEMORY,
    STRUCTURE_TYPE_SAMPLER_CREATE_INFO, SUCCESS,
};

use crate::gpu::vk::Impl;

/// Sampler implementation.
#[derive(Debug)]
pub(super) struct SplrImpl {
    splr: Sampler,
}

impl SplrImpl {
    /// Creates a [`vk_sys::Sampler`].
    fn create_sampler(imp: &Impl, info: &SamplerCreateInfo) -> io::Result<Sampler> {
        let mut splr = vk_sys::null_handle();
        match unsafe {
            imp.dev_fp
                .create_sampler(imp.dev, info, ptr::null(), &mut splr)
        } {
            SUCCESS => Ok(splr),
            ERROR_OUT_OF_DEVICE_MEMORY | ERROR_OUT_OF_HOST_MEMORY => {
                Err(io::Error::from(io::ErrorKind::OutOfMemory))
            }
            _ => Err(io::Error::from(io::ErrorKind::Other)),
        }
    }

    /// Destroys a [`vk_sys::Sampler`].
    fn destroy_sampler(imp: &Impl, splr: Sampler) {
        unsafe {
            imp.dev_fp.destroy_sampler(imp.dev, splr, ptr::null());
        }
    }
}
