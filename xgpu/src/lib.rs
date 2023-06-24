//! xgpu.

mod adapter;
pub use adapter::*;

mod device;
pub use device::*;

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
        _ = dev.create_buffer(/*...*/);
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
}
