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

mod binding;
pub use binding::*;

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

        _ = dev
            .create_texture(&TextureDescriptor {
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
            })
            .unwrap();

        _ = dev.create_sampler(&SamplerDescriptor {
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::Repeat,
            address_mode_w: AddressMode::MirrorRepeat,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Nearest,
            lod_clamp: ..32.0,
            compare: CompareFunction::Never,
            max_anisotropy: 8,
        });

        _ = dev
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStage::Fragment.into(),
                        resource: BindingResourceLayout::Texture {
                            sample_kind: TextureSampleKind::Float,
                            view_dimension: TextureViewDimension::Two,
                            multisampled: false,
                        },
                    },
                    BindGroupLayoutEntry {
                        binding: 2,
                        visibility: ShaderStage::Vertex | ShaderStage::Fragment,
                        resource: BindingResourceLayout::Sampler {
                            kind: SamplerBindingKind::Filtering,
                        },
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStage::Compute.into(),
                        resource: BindingResourceLayout::StorageTexture {
                            access: StorageTextureAccess::WriteOnly,
                            format: TextureFormat::R32Uint,
                            view_dimension: TextureViewDimension::One,
                        },
                    },
                    BindGroupLayoutEntry {
                        binding: 3,
                        visibility: ShaderStage::Vertex.into(),
                        resource: BindingResourceLayout::Buffer {
                            kind: BufferBindingKind::Uniform,
                            has_dynamic_offset: true,
                        },
                    },
                ],
            })
            .unwrap();

        _ = dev.create_pipeline_layout(/*...*/);

        _ = dev.create_bind_group(&BindGroupDescriptor {
            layout: &BindGroupLayout {},
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Texture(&TextureView {}),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::Sampler(&Sampler {}),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::StorageTexture(&TextureView {}),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: BindingResource::Buffer {
                        buffer: &Buffer {},
                        range: 0..256,
                    },
                },
            ],
        });

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
        _ = Sampler {};
    }
}
