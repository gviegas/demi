//! xgpu.

mod adapter;
mod binding;
mod buffer;
mod command;
mod device;
mod error;
mod pipeline;
mod query;
mod queue;
mod sampler;
mod shader;
mod texture;

pub use adapter::*;
pub use binding::*;
pub use buffer::*;
pub use command::*;
pub use device::*;
pub use error::*;
pub use pipeline::*;
pub use query::*;
pub use queue::*;
pub use sampler::*;
pub use shader::*;
pub use texture::*;

// async
pub fn request_adapter(_options: Option<&RequestAdapterOptions>) -> Result<Adapter> {
    panic!("not yet implemented");
}

pub struct RequestAdapterOptions {
    pub power_preference: PowerPreference,
    pub force_fallback_adapter: bool,
}

impl Default for RequestAdapterOptions {
    fn default() -> Self {
        Self {
            power_preference: PowerPreference::LowPower,
            force_fallback_adapter: false,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PowerPreference {
    LowPower,
    HighPerformance,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialization() {
        _ = request_adapter(Some(&RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            force_fallback_adapter: false,
        }));
        _ = request_adapter(Some(&RequestAdapterOptions {
            force_fallback_adapter: true,
            ..Default::default()
        }));
        _ = request_adapter(None);
    }
}
