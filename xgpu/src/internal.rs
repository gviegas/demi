use std::sync::{Arc, RwLock};

pub struct Adapter {
    // TODO
}

pub struct Device {
    // TODO
}

pub struct Buffer {
    device: Arc<RwLock<Device>>,
    // TODO
}

pub struct Texture {
    device: Arc<RwLock<Device>>,
    // TODO
}

pub struct TextureView {
    device: Arc<RwLock<Device>>,
    // TODO
}

pub struct Sampler {
    device: Arc<RwLock<Device>>,
    // TODO
}

pub struct BindGroupLayout {
    device: Arc<RwLock<Device>>,
    // TODO
}

pub struct BindGroup {
    device: Arc<RwLock<Device>>,
    // TODO
}

pub struct PipelineLayout {
    device: Arc<RwLock<Device>>,
    // TODO
}

pub struct ShaderModule {
    device: Arc<RwLock<Device>>,
    // TODO
}

pub struct ComputePipeline {
    device: Arc<RwLock<Device>>,
    // TODO
}

pub struct RenderPipeline {
    device: Arc<RwLock<Device>>,
    // TODO
}

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

pub struct Queue {
    device: Arc<RwLock<Device>>,
    // TODO
}

pub struct QuerySet {
    device: Arc<RwLock<Device>>,
    // TODO
}
