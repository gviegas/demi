//! Internal types.

use crate::{RequestAdapterOptions, Result};

mod adapter;
pub use adapter::*;

mod binding;
pub use binding::*;

mod buffer;
pub use buffer::*;

mod command;
pub use command::*;

mod device;
pub use device::*;

mod pipeline;
pub use pipeline::*;

mod query;
pub use query::*;

mod queue;
pub use queue::*;

mod sampler;
pub use sampler::*;

mod shader;
pub use shader::*;

mod texture;
pub use texture::*;

mod null;

#[cfg(any(target_os = "linux", windows))]
mod vk;

fn new_nadapter(options: &RequestAdapterOptions) -> Result<Box<dyn NAdapter>> {
    #[cfg(any(target_os = "linux", windows))]
    if let Ok(x) = vk::new_adapter(options.power_preference) {
        return Ok(x);
    } else {
        eprintln!("vk::new_adapter failed");
    }

    null::new_adapter()
}
