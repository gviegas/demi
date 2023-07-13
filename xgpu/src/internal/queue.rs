use std::sync::{Arc, RwLock};

use crate::internal::Device;

pub struct Queue {
    device: Arc<RwLock<Device>>,
    // TODO
}
