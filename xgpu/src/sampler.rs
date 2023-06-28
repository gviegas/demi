//! GPU sampler.

use std::ops::RangeBounds;

pub struct Sampler {
    // TODO
}

pub struct SamplerDescriptor<T>
where
    T: RangeBounds<f32>,
{
    pub address_mode_u: AddressMode,
    pub address_mode_v: AddressMode,
    pub address_mode_w: AddressMode,
    pub mag_filter: FilterMode,
    pub min_filter: FilterMode,
    pub mipmap_filter: FilterMode,
    pub lod_clamp: T,
    pub compare: CompareFunction,
    pub max_anisotropy: u16,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AddressMode {
    ClampToEdge,
    Repeat,
    MirrorRepeat,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FilterMode {
    Nearest,
    Linear,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CompareFunction {
    Never,
    Less,
    Equal,
    LessEqual,
    Greater,
    NotEqual,
    GreaterEqual,
    Always,
}
