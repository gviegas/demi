//! xgpu.

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

mod error;
pub use error::*;

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

mod internal;

// TODO: async.
pub fn request_adapter(options: Option<&RequestAdapterOptions>) -> Result<Adapter> {
    Adapter::new(options)
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
