//! GPU adapter.

use crate::{Device, DeviceDescriptor, Result};

pub struct Adapter {
    // TODO
}

impl Adapter {
    pub fn features(&self) -> &SupportedFeatures {
        panic!("not yet implemented");
    }

    pub fn limits(&self) -> &SupportedLimits {
        panic!("not yet implemented");
    }

    pub fn info(&self) -> &AdapterInfo {
        panic!("not yet implemented");
    }

    // async
    pub fn request_device(self, _desc: &DeviceDescriptor) -> Result<Device> {
        panic!("not yet implemented");
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Feature {
    DepthClipControl,
    Depth32FloatStencil8,
    TextureCompressionBc,
    TextureCompressionEtc2,
    TextureCompressionAstc,
    TimestampQuery,
    IndirectFirstInstance,
    ShaderFloat16,
    Rg11b10UfloatRenderable,
    Bgra8UnormStorage,
    Float32Filterable,
}
const MAX_FEATURE: usize = 1 + Feature::Float32Filterable as usize;

#[derive(Clone)]
pub struct SupportedFeatures([bool; MAX_FEATURE]);

impl SupportedFeatures {
    pub fn is_supported(&self, feature: Feature) -> bool {
        self.0[feature as usize]
    }
}

// TODO
pub struct SupportedLimits;

// TODO
pub struct AdapterInfo;
