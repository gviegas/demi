use std::sync::{Arc, RwLock};

use crate::internal::Device;

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
