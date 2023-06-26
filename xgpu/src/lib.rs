//! xgpu.

use std::io;
// TODO
pub type Result<T> = io::Result<T>;

mod adapter;
pub use adapter::*;

mod device;
pub use device::*;

mod buffer;
pub use buffer::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adapter() {
        let adap = Adapter {};
        _ = adap.features();
        _ = adap.limits();
        _ = adap.info();
        _ = adap.request_device(&DeviceDescriptor {});
    }

    #[test]
    fn device() {
        let mut dev = Device {};
        _ = dev.features();
        _ = dev.limits();
        _ = dev.queue();
        _ = dev.create_buffer(&BufferDescriptor {
            size: 16384,
            usage: BufferUsage::CopyDst | BufferUsage::QueryResolve | BufferUsage::Storage,
            mapped_at_creation: false,
        });
        _ = dev.create_texture(/*...*/);
        _ = dev.create_sampler(/*...*/);
        _ = dev.create_bind_group_layout(/*...*/);
        _ = dev.create_pipeline_layout(/*...*/);
        _ = dev.create_bind_group(/*...*/);
        _ = dev.create_shader_module(/*...*/);
        _ = dev.create_compute_pipeline(/*...*/);
        _ = dev.create_render_pipeline(/*...*/);
        _ = dev.create_command_encoder(/*...*/);
        _ = dev.create_render_bundle_encoder(/*...*/);
        _ = dev.create_query_set(/*...*/);
    }

    #[test]
    fn buffer() {
        let mut buf = Buffer {};
        _ = buf.size();
        _ = buf.usage();
        _ = buf.map_state();
        _ = buf.map(MapMode::Read, ..);
        _ = buf.get_mapped_range(256..512);
        _ = buf.unmap();
    }
}
