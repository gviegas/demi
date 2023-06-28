//! GPU resource binding.

use std::ops::Range;

use crate::{Buffer, Sampler, ShaderStageFlags, TextureFormat, TextureView, TextureViewDimension};

pub struct BindGroupLayout {
    // TODO
}

pub struct BindGroupLayoutDescriptor<'a> {
    pub entries: &'a [BindGroupLayoutEntry],
}

pub struct BindGroupLayoutEntry {
    pub binding: u32,
    pub visibility: ShaderStageFlags,
    pub resource: BindingResourceLayout,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BindingResourceLayout {
    Buffer {
        kind: BufferBindingKind,
        has_dynamic_offset: bool,
        //min_binding_size: u64,
    },
    Sampler {
        kind: SamplerBindingKind,
    },
    Texture {
        sample_kind: TextureSampleKind,
        view_dimension: TextureViewDimension,
        multisampled: bool,
    },
    StorageTexture {
        access: StorageTextureAccess,
        format: TextureFormat,
        view_dimension: TextureViewDimension,
    },
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BufferBindingKind {
    Uniform,
    Storage,
    ReadOnlyStorage,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SamplerBindingKind {
    Filtering,
    NonFiltering,
    Comparison,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TextureSampleKind {
    Float,
    UnfilterableFloat,
    Depth,
    Sint,
    Uint,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StorageTextureAccess {
    WriteOnly,
}

pub struct BindGroup {
    // TODO
}

// TODO: Ensure that `BindGroup` doesn't outlive its resources.
// Maybe use `Arc` for resources instead.
pub struct BindGroupDescriptor<'a, 'b, 'c: 'b> {
    pub layout: &'a BindGroupLayout,
    pub entries: &'b [BindGroupEntry<'c>],
}

pub struct BindGroupEntry<'a> {
    pub binding: u32,
    pub resource: BindingResource<'a>,
}

#[derive(Clone)]
pub enum BindingResource<'a> {
    Buffer {
        buffer: &'a Buffer,
        range: Range<u64>,
    },
    Sampler(&'a Sampler),
    Texture(&'a TextureView),
    StorageTexture(&'a TextureView),
}

pub struct PipelineLayout {
    // TODO
}

pub struct PipelineLayoutDescriptor<'a, 'b: 'a> {
    pub bind_group_layouts: &'a [&'b BindGroupLayout],
}
