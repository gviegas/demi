use std::sync::{Arc, RwLock};

use crate::internal::Device;

pub struct CommandBuffer {
    device: Arc<RwLock<Device>>,
    // TODO
}

pub struct CommandEncoder {
    device: Arc<RwLock<Device>>,
    // TODO
}

pub struct ComputePassEncoder {
    device: Arc<RwLock<Device>>,
    // TODO
}

pub struct RenderPassEncoder {
    device: Arc<RwLock<Device>>,
    // TODO
}

pub struct RenderBundle {
    device: Arc<RwLock<Device>>,
    // TODO
}

pub struct RenderBundleEncoder {
    device: Arc<RwLock<Device>>,
    // TODO
}
