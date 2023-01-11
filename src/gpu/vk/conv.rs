// Copyright 2023 Gustavo C. Viegas. All rights reserved.

use std::mem;

use vk_sys::{
    CompareOp, ComponentMapping, FormatFeatureFlags, ImageAspectFlags, InstanceFp, PhysicalDevice,
    PrimitiveTopology, SampleCountFlagBits, SamplerAddressMode, SamplerMipmapMode,
    COMPARE_OP_ALWAYS, COMPARE_OP_EQUAL, COMPARE_OP_GREATER, COMPARE_OP_GREATER_OR_EQUAL,
    COMPARE_OP_LESS, COMPARE_OP_LESS_OR_EQUAL, COMPARE_OP_NEVER, COMPARE_OP_NOT_EQUAL,
    COMPONENT_SWIZZLE_A, COMPONENT_SWIZZLE_B, COMPONENT_SWIZZLE_G, COMPONENT_SWIZZLE_IDENTITY,
    COMPONENT_SWIZZLE_ONE, COMPONENT_SWIZZLE_R, FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT,
    FORMAT_FEATURE_SAMPLED_IMAGE_BIT, IMAGE_ASPECT_COLOR_BIT, IMAGE_ASPECT_DEPTH_BIT,
    IMAGE_ASPECT_STENCIL_BIT, LOD_CLAMP_NONE, PRIMITIVE_TOPOLOGY_LINE_LIST,
    PRIMITIVE_TOPOLOGY_LINE_STRIP, PRIMITIVE_TOPOLOGY_POINT_LIST, PRIMITIVE_TOPOLOGY_TRIANGLE_FAN,
    PRIMITIVE_TOPOLOGY_TRIANGLE_LIST, PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP,
    SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE, SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT,
    SAMPLER_ADDRESS_MODE_REPEAT, SAMPLER_MIPMAP_MODE_LINEAR, SAMPLER_MIPMAP_MODE_NEAREST,
    SAMPLE_COUNT_16_BIT, SAMPLE_COUNT_1_BIT, SAMPLE_COUNT_2_BIT, SAMPLE_COUNT_32_BIT,
    SAMPLE_COUNT_4_BIT, SAMPLE_COUNT_64_BIT, SAMPLE_COUNT_8_BIT,
};

use crate::mesh::{DataType, Topology};
use crate::sampler::{self, Compare, Wrap};
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
    pub fn convert(&self, fmt: texture::Format) -> (vk_sys::Format, ComponentMapping) {
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

/// Converts from a sample count into a [`vk_sys::SampleCountFlagBits`].
pub(super) fn from_sample_count(count: u32) -> SampleCountFlagBits {
    match count {
        0..=1 => SAMPLE_COUNT_1_BIT,
        2 => SAMPLE_COUNT_2_BIT,
        4 => SAMPLE_COUNT_4_BIT,
        8 => SAMPLE_COUNT_8_BIT,
        16 => SAMPLE_COUNT_16_BIT,
        32 => SAMPLE_COUNT_32_BIT,
        64.. => SAMPLE_COUNT_64_BIT,
        _ => panic!("gpu::vk: unexpected sample count ({})", count),
    }
}

/// Returns the [`vk_sys::ImageAspectFlags`] of a given
/// [`texture::Format`].
pub(super) fn aspect_of(fmt: texture::Format) -> ImageAspectFlags {
    match fmt {
        texture::Format::Xrgb8888
        | texture::Format::Argb8888
        | texture::Format::Rgba8888
        | texture::Format::Bgra8888
        | texture::Format::Rgba16161616
        | texture::Format::GenericLdr
        | texture::Format::GenericHdr
        | texture::Format::CompressedLdr
        | texture::Format::CompressedHdr => IMAGE_ASPECT_COLOR_BIT,
        texture::Format::GenericDepth => IMAGE_ASPECT_DEPTH_BIT,
        texture::Format::GenericDepthStencil => IMAGE_ASPECT_DEPTH_BIT | IMAGE_ASPECT_STENCIL_BIT,
    }
}

/// Converts from a [`Wrap`] into a [`vk_sys::SamplerAddressMode`].
pub(super) fn from_wrap_mode(wrap: Wrap) -> SamplerAddressMode {
    match wrap {
        Wrap::Repeat => SAMPLER_ADDRESS_MODE_REPEAT,
        Wrap::MirroredRepeat => SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT,
        Wrap::ClampToEdge => SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE,
    }
}

/// Converts from a magnification [`sampler::Filter`] into a
/// [`vk_sys::Filter`].
pub(super) fn from_mag_filter(filter: sampler::Filter) -> vk_sys::Filter {
    match filter {
        sampler::Filter::Nearest => vk_sys::FILTER_NEAREST,
        sampler::Filter::Linear => vk_sys::FILTER_LINEAR,
    }
}

/// Converts from a minification [`sampler::Filter`] pair into a tuple
/// containing [`vk_sys::Filter`], [`vk_sys::SamplerMipmapMode`],
/// min LOD and max LOD values.
pub(super) fn from_min_filter(
    filter: sampler::Filter,
    mipmap: Option<sampler::Filter>,
) -> (vk_sys::Filter, SamplerMipmapMode, f32, f32) {
    let min = match filter {
        sampler::Filter::Nearest => vk_sys::FILTER_NEAREST,
        sampler::Filter::Linear => vk_sys::FILTER_LINEAR,
    };
    let (mip, max_lod) = if let Some(x) = mipmap {
        match x {
            sampler::Filter::Nearest => (SAMPLER_MIPMAP_MODE_NEAREST, LOD_CLAMP_NONE),
            sampler::Filter::Linear => (SAMPLER_MIPMAP_MODE_LINEAR, LOD_CLAMP_NONE),
        }
    } else {
        (SAMPLER_MIPMAP_MODE_NEAREST, 0.25)
    };
    (min, mip, 0.0, max_lod)
}

/// Converts from a [`Compare`] into a [`vk_sys::CompareOp`].
pub(super) fn from_compare_fn(compare: Compare) -> CompareOp {
    match compare {
        Compare::Never => COMPARE_OP_NEVER,
        Compare::Less => COMPARE_OP_LESS,
        Compare::LessEqual => COMPARE_OP_LESS_OR_EQUAL,
        Compare::Equal => COMPARE_OP_EQUAL,
        Compare::NotEqual => COMPARE_OP_NOT_EQUAL,
        Compare::GreaterEqual => COMPARE_OP_GREATER_OR_EQUAL,
        Compare::Greater => COMPARE_OP_GREATER,
        Compare::Always => COMPARE_OP_ALWAYS,
    }
}

/// Converts from a [`DataType`] into a [`vk_sys::Format`].
///
/// NOTE: The following conversions generate [`vk_sys::Format`]s that
/// may not support vertex buffer usage:
///
/// - [`DataType::U16x3`]
/// - [`DataType::U8x3`]
pub(super) fn from_data_type(data_type: DataType) -> vk_sys::Format {
    match data_type {
        DataType::F32 => vk_sys::FORMAT_R32_SFLOAT,
        DataType::F32x2 => vk_sys::FORMAT_R32G32_SFLOAT,
        DataType::F32x3 => vk_sys::FORMAT_R32G32B32_SFLOAT,
        DataType::F32x4 => vk_sys::FORMAT_R32G32B32A32_SFLOAT,
        DataType::U32 => vk_sys::FORMAT_R32_UINT,
        DataType::U32x2 => vk_sys::FORMAT_R32G32_UINT,
        DataType::U32x3 => vk_sys::FORMAT_R32G32B32_UINT,
        DataType::U32x4 => vk_sys::FORMAT_R32G32B32A32_UINT,
        DataType::U16 => vk_sys::FORMAT_R16_UINT,
        DataType::U16x2 => vk_sys::FORMAT_R16G16_UINT,
        DataType::U16x3 => vk_sys::FORMAT_R16G16B16_UINT,
        DataType::U16x4 => vk_sys::FORMAT_R16G16B16A16_UINT,
        DataType::U8 => vk_sys::FORMAT_R8_UINT,
        DataType::U8x2 => vk_sys::FORMAT_R8G8_UINT,
        DataType::U8x3 => vk_sys::FORMAT_R8G8B8_UINT,
        DataType::U8x4 => vk_sys::FORMAT_R8G8B8A8_UINT,
    }
}

/// Converts from a [`Topology`] into a [`vk_sys::PrimitiveTopology`].
pub(super) fn from_topology(topology: Topology) -> PrimitiveTopology {
    match topology {
        Topology::Point => PRIMITIVE_TOPOLOGY_POINT_LIST,
        Topology::Line => PRIMITIVE_TOPOLOGY_LINE_LIST,
        Topology::LineStrip => PRIMITIVE_TOPOLOGY_LINE_STRIP,
        Topology::Triangle => PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
        Topology::TriangleStrip => PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP,
        Topology::TriangleFan => PRIMITIVE_TOPOLOGY_TRIANGLE_FAN,
    }
}
