// Copyright 2023 Gustavo C. Viegas. All rights reserved.

use std::io;
use std::mem;
use std::ptr;

use vk_sys::{
    DeviceMemory, Image, ImageCreateInfo, ERROR_OUT_OF_DEVICE_MEMORY, ERROR_OUT_OF_HOST_MEMORY,
    STRUCTURE_TYPE_IMAGE_CREATE_INFO, SUCCESS,
};

use crate::gpu::vk::Impl;

/// Texture implementation.
#[derive(Debug)]
pub(super) struct TexImpl {
    img: Image,
    mem: DeviceMemory,
}

impl TexImpl {
    /// Creates a [`vk_sys::Image`].
    fn create_image(imp: &Impl, info: &ImageCreateInfo) -> io::Result<Image> {
        let mut img = ptr::null_mut();
        match unsafe {
            imp.dev_fp
                .create_image(imp.dev, info, ptr::null(), &mut img)
        } {
            SUCCESS => Ok(img),
            ERROR_OUT_OF_DEVICE_MEMORY | ERROR_OUT_OF_HOST_MEMORY => {
                Err(io::Error::from(io::ErrorKind::OutOfMemory))
            }
            _ => Err(io::Error::from(io::ErrorKind::Other)),
        }
    }

    /// Destroys a [`vk_sys::Image`].
    fn destroy_image(imp: &Impl, img: Image) {
        unsafe {
            imp.dev_fp.destroy_image(imp.dev, img, ptr::null());
        }
    }

    /// Binds a [`vk_sys::Image`] to newly allocated memory.
    ///
    /// NOTE: This memory cannot be mapped for host access.
    fn bind(imp: &Impl, img: Image) -> io::Result<DeviceMemory> {
        unsafe {
            let mut req = mem::zeroed();
            imp.dev_fp
                .get_image_memory_requirements(imp.dev, img, &mut req);
            // Image memory is always GPU-private.
            let mem = imp.alloc(&req, false)?;
            match imp.dev_fp.bind_image_memory(imp.dev, img, mem, 0) {
                SUCCESS => Ok(mem),
                ERROR_OUT_OF_DEVICE_MEMORY | ERROR_OUT_OF_HOST_MEMORY => {
                    imp.dealloc(mem);
                    Err(io::Error::from(io::ErrorKind::OutOfMemory))
                }
                _ => {
                    imp.dealloc(mem);
                    Err(io::Error::from(io::ErrorKind::Other))
                }
            }
        }
    }
}
