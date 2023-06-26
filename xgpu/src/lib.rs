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

mod texture;
pub use texture::*;

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
        _ = dev.create_texture(&TextureDescriptor {
            size: Extent3d {
                width: 1024,
                height: 1024,
                depth_or_layers: 1,
            },
            level_count: 11,
            sample_count: 1,
            dimension: TextureDimension::Two,
            format: TextureFormat::Rgba8Unorm,
            usage: TextureUsage::CopyDst
                | TextureUsage::TextureBinding
                | TextureUsage::RenderAttachment,
            view_formats: &[TextureFormat::R8Unorm, TextureFormat::Rg16Float],
        });
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

    #[test]
    fn texture() {
        let mut tex = Texture {};
        _ = tex.width();
        _ = tex.height();
        _ = tex.depth_or_layers();
        _ = tex.level_count();
        _ = tex.sample_count();
        _ = tex.dimension();
        _ = tex.format();
        _ = tex.usage();
        _ = tex.create_view(&TextureViewDescriptor {
            format: TextureFormat::Rgba8UnormSrgb,
            dimension: TextureViewDimension::TwoArray,
            aspect: TextureAspect::All,
            level_range: ..,
            layer_range: 4..,
        });
    }
}
