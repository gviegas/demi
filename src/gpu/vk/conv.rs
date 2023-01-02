// Copyright 2023 Gustavo C. Viegas. All rights reserved.

use std::mem;

use vk_sys::{
    ComponentMapping, FormatFeatureFlags, InstanceFp, PhysicalDevice, COMPONENT_SWIZZLE_A,
    COMPONENT_SWIZZLE_B, COMPONENT_SWIZZLE_G, COMPONENT_SWIZZLE_IDENTITY, COMPONENT_SWIZZLE_ONE,
    COMPONENT_SWIZZLE_R, FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT,
    FORMAT_FEATURE_SAMPLED_IMAGE_BIT,
};

use crate::texture;

/// Format converter.
///
/// This type handles device-specific format support
/// (mainly for depth/stencil) and component swizzle for
/// [`texture::Format`]s that do not have an exactly
/// match in Vulkan.
#[derive(Debug)]
pub(super) struct FmtConv {
    depth: vk_sys::Format,
    depth_stencil: vk_sys::Format,
}

impl FmtConv {
    /// Creates a new format converter.
    pub fn new(dev: PhysicalDevice, fp: &InstanceFp) -> Self {
        const DEPTH: [vk_sys::Format; 3] = [
            vk_sys::FORMAT_X8_D24_UNORM_PACK32,
            vk_sys::FORMAT_D32_SFLOAT,
            vk_sys::FORMAT_D16_UNORM,
        ];
        const DEPTH_STENCIL: [vk_sys::Format; 3] = [
            vk_sys::FORMAT_D24_UNORM_S8_UINT,
            vk_sys::FORMAT_D32_SFLOAT_S8_UINT,
            vk_sys::FORMAT_D16_UNORM_S8_UINT,
        ];
        const FLAGS: [FormatFeatureFlags; 2] = [
            FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT | FORMAT_FEATURE_SAMPLED_IMAGE_BIT,
            FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT,
        ];

        let get_fmt = |fmts, flags| unsafe {
            for i in fmts {
                let mut prop = mem::zeroed();
                fp.get_physical_device_format_properties(dev, i, &mut prop);
                if flags & prop.optimal_tiling_features == flags {
                    return Some(i);
                }
            }
            None
        };

        // NOTE: This should never panic.
        Self {
            depth: get_fmt(DEPTH, FLAGS[0]).unwrap(),
            depth_stencil: get_fmt(DEPTH_STENCIL, FLAGS[0])
                .or_else(|| get_fmt(DEPTH_STENCIL, FLAGS[1]))
                .unwrap(),
        }
    }

    /// Converts from a [`texture::Format`] into a [`vk_sys::Format`]
    /// with possibly remapped components.
    ///
    /// NOTE: Formats that require remapping of components must only
    /// be used to create sampled textures.
    pub fn from_texture_format(&self, fmt: texture::Format) -> (vk_sys::Format, ComponentMapping) {
        const IDENTITY: ComponentMapping = ComponentMapping {
            r: COMPONENT_SWIZZLE_IDENTITY,
            g: COMPONENT_SWIZZLE_IDENTITY,
            b: COMPONENT_SWIZZLE_IDENTITY,
            a: COMPONENT_SWIZZLE_IDENTITY,
        };

        match fmt {
            texture::Format::Xrgb8888 => (
                vk_sys::FORMAT_R8G8B8A8_UNORM,
                ComponentMapping {
                    r: COMPONENT_SWIZZLE_G,
                    g: COMPONENT_SWIZZLE_B,
                    b: COMPONENT_SWIZZLE_A,
                    a: COMPONENT_SWIZZLE_ONE,
                },
            ),
            texture::Format::Argb8888 => (
                vk_sys::FORMAT_R8G8B8A8_UNORM,
                ComponentMapping {
                    r: COMPONENT_SWIZZLE_G,
                    g: COMPONENT_SWIZZLE_B,
                    b: COMPONENT_SWIZZLE_A,
                    a: COMPONENT_SWIZZLE_R,
                },
            ),
            texture::Format::Rgba8888 | texture::Format::GenericLdr => {
                (vk_sys::FORMAT_R8G8B8A8_UNORM, IDENTITY)
            }
            texture::Format::Bgra8888 => (vk_sys::FORMAT_B8G8R8A8_UNORM, IDENTITY),
            texture::Format::Rgba16161616 => (vk_sys::FORMAT_R16G16B16A16_SFLOAT, IDENTITY),
            texture::Format::GenericHdr => (vk_sys::FORMAT_A2B10G10R10_UNORM_PACK32, IDENTITY),
            texture::Format::GenericDepth => (self.depth, IDENTITY),
            texture::Format::GenericDepthStencil => (self.depth_stencil, IDENTITY),
            texture::Format::CompressedLdr => todo!(),
            texture::Format::CompressedHdr => todo!(),
        }
    }
}
