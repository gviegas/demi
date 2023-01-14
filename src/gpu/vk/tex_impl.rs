// Copyright 2023 Gustavo C. Viegas. All rights reserved.

use std::io;
use std::mem;
use std::ptr::{self, NonNull};

use vk_sys::{
    DeviceMemory, Extent3d, Image, ImageCreateInfo, ERROR_OUT_OF_DEVICE_MEMORY,
    ERROR_OUT_OF_HOST_MEMORY, IMAGE_ASPECT_COLOR_BIT, IMAGE_ASPECT_DEPTH_BIT,
    IMAGE_ASPECT_STENCIL_BIT, IMAGE_CREATE_CUBE_COMPATIBLE_BIT, IMAGE_LAYOUT_UNDEFINED,
    IMAGE_TILING_OPTIMAL, IMAGE_TYPE_2D, IMAGE_TYPE_3D, IMAGE_USAGE_COLOR_ATTACHMENT_BIT,
    IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT, IMAGE_USAGE_SAMPLED_BIT,
    IMAGE_USAGE_TRANSFER_DST_BIT, IMAGE_USAGE_TRANSFER_SRC_BIT, SAMPLE_COUNT_1_BIT,
    SHARING_MODE_EXCLUSIVE, STRUCTURE_TYPE_IMAGE_CREATE_INFO, SUCCESS,
};

use crate::gpu::vk::conv;
use crate::gpu::vk::Impl;
use crate::gpu::{Id, TexId, TexOptions};

/// Texture implementation.
///
/// This type represents a [`TexId`].
#[derive(Debug)]
pub(super) struct TexImpl {
    img: Image,
    mem: DeviceMemory,
}

// TODO: Missing parameter validation, format support and
// limit checks on `new_*` functions.
impl TexImpl {
    /// Creates a [`vk_sys::Image`].
    fn create_image(imp: &Impl, info: &ImageCreateInfo) -> io::Result<Image> {
        let mut img = vk_sys::null_handle();
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

    /// Creates a new [`TexImpl`] to use as a 2D texture.
    ///
    /// It supports sampling in shaders and copying.
    pub fn new_2d(imp: &Impl, options: &TexOptions) -> io::Result<Self> {
        let info = ImageCreateInfo {
            s_type: STRUCTURE_TYPE_IMAGE_CREATE_INFO,
            next: ptr::null(),
            flags: 0,
            image_type: IMAGE_TYPE_2D,
            format: imp.fmt_conv.convert(options.format).0,
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

    /// Creates a new [`TexImpl`] to use as a 3D texture.
    ///
    /// It supports sampling in shaders and copying.
    pub fn new_3d(imp: &Impl, options: &TexOptions) -> io::Result<Self> {
        let info = ImageCreateInfo {
            s_type: STRUCTURE_TYPE_IMAGE_CREATE_INFO,
            next: ptr::null(),
            flags: 0,
            image_type: IMAGE_TYPE_3D,
            format: imp.fmt_conv.convert(options.format).0,
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

    /// Creates a new [`TexImpl`] to use as a cube texture.
    ///
    /// It supports sampling in shaders and copying.
    pub fn new_cube(imp: &Impl, options: &TexOptions) -> io::Result<Self> {
        // TODO: Cube array need special care, since it is a feature.
        let info = ImageCreateInfo {
            s_type: STRUCTURE_TYPE_IMAGE_CREATE_INFO,
            next: ptr::null(),
            flags: IMAGE_CREATE_CUBE_COMPATIBLE_BIT,
            image_type: IMAGE_TYPE_2D,
            format: imp.fmt_conv.convert(options.format).0,
            extent: Extent3d {
                width: options.width,
                height: options.height,
                depth: 1,
            },
            mip_levels: options.levels,  // TODO: May be incorrect.
            array_layers: options.depth, // TODO: May be incorrect.
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

    /// Creates a new [`TexImpl`] to use as a render target texture.
    ///
    /// It supports sampling in shaders, copying and use as
    /// either color or depth/stencil attachment, depending
    /// on the `options.format`.
    pub fn new_rt(imp: &Impl, options: &TexOptions) -> io::Result<Self> {
        let usage = IMAGE_USAGE_SAMPLED_BIT
            | IMAGE_USAGE_TRANSFER_SRC_BIT
            | IMAGE_USAGE_TRANSFER_DST_BIT
            | match conv::aspect_of(options.format) {
                IMAGE_ASPECT_COLOR_BIT => IMAGE_USAGE_COLOR_ATTACHMENT_BIT,
                x if x & (IMAGE_ASPECT_DEPTH_BIT | IMAGE_ASPECT_STENCIL_BIT) != 0 => {
                    IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT
                }
                _ => {
                    // NOTE: This should be unreachable.
                    return Err(io::Error::from(io::ErrorKind::Unsupported));
                }
            };
        let info = ImageCreateInfo {
            s_type: STRUCTURE_TYPE_IMAGE_CREATE_INFO,
            next: ptr::null(),
            flags: 0,
            image_type: IMAGE_TYPE_2D,
            format: imp.fmt_conv.convert(options.format).0,
            extent: Extent3d {
                width: options.width,
                height: options.height,
                depth: 1,
            },
            mip_levels: 1,
            array_layers: options.depth,
            samples: conv::from_sample_count(options.samples), // TODO: May not be supported.
            tiling: IMAGE_TILING_OPTIMAL,
            usage,
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

    /// Destroys the [`TexImpl`].
    pub fn drop_with(self, imp: &Impl) {
        Self::destroy_image(imp, self.img);
        imp.dealloc(self.mem);
    }
}

impl From<TexId> for Box<TexImpl> {
    /// Converts from a [`TexId`] into a boxed [`TexImpl`].
    fn from(tex_id: TexId) -> Self {
        let non_null = match tex_id.0 {
            Id::Ptr(x) => x,
            _ => unreachable!(),
        };
        let raw_ptr = non_null.as_ptr() as *mut TexImpl;
        unsafe { Box::from_raw(raw_ptr) }
    }
}

impl From<&TexId> for &TexImpl {
    /// Converts from a &[`TexId`] into a &[`TexImpl`].
    fn from(tex_id: &TexId) -> Self {
        let non_null = match tex_id.0 {
            Id::Ptr(x) => x,
            _ => unreachable!(),
        };
        unsafe { non_null.cast().as_ref() }
    }
}

impl From<Box<TexImpl>> for TexId {
    /// Converts from a boxed [`TexImpl`] into a [`TexId`].
    fn from(tex_imp: Box<TexImpl>) -> Self {
        let raw_ptr = Box::into_raw(tex_imp) as *mut ();
        let non_null = unsafe { NonNull::new_unchecked(raw_ptr) };
        TexId(Id::Ptr(non_null))
    }
}

#[cfg(test)]
mod tests {
    use super::TexImpl;
    use crate::gpu::{self, TexId, TexOptions};
    use crate::texture;

    #[test]
    fn new() {
        crate::init();

        let assert = |tex_imp: &TexImpl| {
            assert!(!vk_sys::is_null_handle(tex_imp.img));
            assert!(!vk_sys::is_null_handle(tex_imp.mem));
        };

        // 2D layer=1 level=1 no MS.
        let options = TexOptions {
            format: texture::Format::Rgba8888,
            width: 1024,
            height: 1024,
            depth: 1,
            levels: 1,
            samples: 1,
        };
        let tex_imp = Box::<TexImpl>::from(gpu::create_rt(&options).unwrap());
        assert(&tex_imp);
        gpu::drop_texture(&mut TexId::from(tex_imp));

        // 2D layer>1 level=1 no MS.
        let options = TexOptions {
            format: texture::Format::Xrgb8888,
            width: 1024,
            height: 1024,
            depth: 16,
            levels: 1,
            samples: 1,
        };
        let tex_imp = Box::<TexImpl>::from(gpu::create_rt(&options).unwrap());
        assert(&tex_imp);
        gpu::drop_texture(&mut TexId::from(tex_imp));

        // 2D layer>1 level>1 no MS.
        let options = TexOptions {
            format: texture::Format::Bgra8888,
            width: 512,
            height: 512,
            depth: 3,
            levels: 10,
            samples: 1,
        };
        let tex_imp = Box::<TexImpl>::from(gpu::create_rt(&options).unwrap());
        assert(&tex_imp);
        gpu::drop_texture(&mut TexId::from(tex_imp));

        // 3D level=1.
        let options = TexOptions {
            format: texture::Format::Rgba8888,
            width: 512,
            height: 512,
            depth: 64,
            levels: 1,
            samples: 1,
        };
        let tex_imp = Box::<TexImpl>::from(gpu::create_rt(&options).unwrap());
        assert(&tex_imp);
        gpu::drop_texture(&mut TexId::from(tex_imp));

        // Cube layer=1(6) level=1 no MS.
        let options = TexOptions {
            format: texture::Format::Argb8888,
            width: 640,
            height: 640,
            depth: 6,
            levels: 1,
            samples: 1,
        };
        let tex_imp = Box::<TexImpl>::from(gpu::create_rt(&options).unwrap());
        assert(&tex_imp);
        gpu::drop_texture(&mut TexId::from(tex_imp));

        // Color (LDR) layer=1 4x MS.
        let options = TexOptions {
            format: texture::Format::GenericLdr,
            width: 1280,
            height: 720,
            depth: 1,
            levels: 1,
            samples: 4,
        };
        let tex_imp = Box::<TexImpl>::from(gpu::create_rt(&options).unwrap());
        assert(&tex_imp);
        gpu::drop_texture(&mut TexId::from(tex_imp));

        // Color (HDR) layer=1 4x MS.
        let options = TexOptions {
            format: texture::Format::GenericHdr,
            width: 1920,
            height: 1080,
            depth: 1,
            levels: 1,
            samples: 4,
        };
        let tex_imp = Box::<TexImpl>::from(gpu::create_rt(&options).unwrap());
        assert(&tex_imp);
        gpu::drop_texture(&mut TexId::from(tex_imp));

        // Depth layer=1 no MS.
        let options = TexOptions {
            format: texture::Format::GenericDepth,
            width: 1600,
            height: 900,
            depth: 1,
            levels: 1,
            samples: 4,
        };
        let tex_imp = Box::<TexImpl>::from(gpu::create_rt(&options).unwrap());
        assert(&tex_imp);
        gpu::drop_texture(&mut TexId::from(tex_imp));

        // Depth/stencil layer=1 no MS.
        let options = TexOptions {
            format: texture::Format::GenericDepthStencil,
            width: 1280,
            height: 800,
            depth: 1,
            levels: 1,
            samples: 4,
        };
        let tex_imp = Box::<TexImpl>::from(gpu::create_rt(&options).unwrap());
        assert(&tex_imp);
        gpu::drop_texture(&mut TexId::from(tex_imp));

        crate::shutdown();
    }
}
