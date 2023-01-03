// Copyright 2023 Gustavo C. Viegas. All rights reserved.

use std::io;
use std::mem;
use std::ptr;

use vk_sys::{
    DeviceMemory, Extent3d, Image, ImageCreateInfo, ERROR_OUT_OF_DEVICE_MEMORY,
    ERROR_OUT_OF_HOST_MEMORY, IMAGE_LAYOUT_UNDEFINED, IMAGE_TILING_OPTIMAL, IMAGE_TYPE_2D,
    IMAGE_TYPE_3D, IMAGE_USAGE_SAMPLED_BIT, IMAGE_USAGE_TRANSFER_DST_BIT,
    IMAGE_USAGE_TRANSFER_SRC_BIT, SAMPLE_COUNT_1_BIT, SHARING_MODE_EXCLUSIVE,
    STRUCTURE_TYPE_IMAGE_CREATE_INFO, SUCCESS,
};

use crate::gpu::vk::conv;
use crate::gpu::vk::Impl;
use crate::gpu::TexOptions;

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

    /// Creates a new `TexImpl` to use as a 2D texture.
    ///
    /// It supports sampling in shaders and copying.
    pub fn new_2d(imp: &Impl, options: &TexOptions) -> io::Result<Self> {
        let info = ImageCreateInfo {
            s_type: STRUCTURE_TYPE_IMAGE_CREATE_INFO,
            next: ptr::null(),
            flags: 0,
            image_type: IMAGE_TYPE_2D,
            format: imp.fmt_conv.from_texture_format(options.format).0,
            extent: Extent3d {
                width: options.width,
                height: options.height,
                depth: 1,
            },
            mip_levels: options.levels, // TODO: May be incorrect.
            array_layers: options.depth,
            samples: conv::from_sample_count(options.samples), // TODO: May be unsupported.
            tiling: IMAGE_TILING_OPTIMAL,
            usage: IMAGE_USAGE_SAMPLED_BIT
                | IMAGE_USAGE_TRANSFER_SRC_BIT
                | IMAGE_USAGE_TRANSFER_DST_BIT,
            sharing_mode: SHARING_MODE_EXCLUSIVE,
            queue_family_index_count: 0,
            queue_family_indices: ptr::null(),
            initial_layout: IMAGE_LAYOUT_UNDEFINED,
        };
        let img = Self::create_image(imp, &info)?;
        match Self::bind(imp, img) {
            Ok(mem) => Ok(Self { img, mem }),
            Err(e) => {
                Self::destroy_image(imp, img);
                Err(e)
            }
        }
    }

    /// Creates a new `TexImpl` to use as a 3D texture.
    ///
    /// It supports sampling in shaders and copying.
    pub fn new_3d(imp: &Impl, options: &TexOptions) -> io::Result<Self> {
        let info = ImageCreateInfo {
            s_type: STRUCTURE_TYPE_IMAGE_CREATE_INFO,
            next: ptr::null(),
            flags: 0,
            image_type: IMAGE_TYPE_3D,
            format: imp.fmt_conv.from_texture_format(options.format).0,
            extent: Extent3d {
                width: options.width,
                height: options.height,
                depth: options.depth,
            },
            mip_levels: options.levels, // TODO: May be incorrect.
            array_layers: 1,
            samples: SAMPLE_COUNT_1_BIT,
            tiling: IMAGE_TILING_OPTIMAL,
            usage: IMAGE_USAGE_SAMPLED_BIT
                | IMAGE_USAGE_TRANSFER_SRC_BIT
                | IMAGE_USAGE_TRANSFER_DST_BIT,
            sharing_mode: SHARING_MODE_EXCLUSIVE,
            queue_family_index_count: 0,
            queue_family_indices: ptr::null(),
            initial_layout: IMAGE_LAYOUT_UNDEFINED,
        };
        let img = Self::create_image(imp, &info)?;
        match Self::bind(imp, img) {
            Ok(mem) => Ok(Self { img, mem }),
            Err(e) => {
                Self::destroy_image(imp, img);
                Err(e)
            }
        }
    }
}
