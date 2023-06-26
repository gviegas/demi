//! GPU resource binding.

use std::ops::{BitOr, Range};

use crate::{Buffer, Sampler, TextureFormat, TextureView, TextureViewDimension};

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

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ShaderStage {
    Vertex = 0x1,
    Fragment = 0x2,
    Compute = 0x4,
}

impl BitOr for ShaderStage {
    type Output = ShaderStageFlags;

    fn bitor(self, rhs: Self) -> Self::Output {
        ShaderStageFlags(self as u16 | rhs as u16)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ShaderStageFlags(u16);

impl ShaderStageFlags {
    pub fn is_set(self, stage: ShaderStage) -> bool {
        self.0 & stage as u16 != 0
    }
}

impl BitOr<ShaderStage> for ShaderStageFlags {
    type Output = Self;

    fn bitor(self, rhs: ShaderStage) -> Self::Output {
        Self(self.0 | rhs as u16)
    }
}

impl From<ShaderStage> for ShaderStageFlags {
    fn from(value: ShaderStage) -> Self {
        Self(value as u16)
    }
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
