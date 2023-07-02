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
pub fn request_adapter(_options: &RequestAdapterOptions) -> Result<Adapter> {
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
        _ = request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            force_fallback_adapter: false,
        });
        _ = request_adapter(&Default::default());
    }

    #[test]
    fn adapter() {
        let adap = Adapter {};
        _ = adap.features();
        _ = adap.limits();
        _ = adap.is_fallback_adapter();
        _ = adap.request_adapter_info();
        _ = adap.request_device(&DeviceDescriptor {
            required_features: &[
                Feature::TextureCompressionBc,
                Feature::Depth32FloatStencil8,
                Feature::TimestampQuery,
            ],
            required_limits: &[
                Limit::MaxBindingsPerBindGroup(60),
                Limit::MaxVertexBuffers(14),
                Limit::MaxColorAttachments(10),
            ],
            default_queue: QueueDescriptor {},
        });
        let adap = Adapter {};
        _ = adap.request_device(&Default::default());
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
            mipmap_filter: MipmapFilterMode::Nearest,
            lod_clamp: ..32.0,
            compare: None,
            max_anisotropy: 8,
        });
        _ = dev.create_sampler(&Default::default());

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
                            min_binding_size: 256,
                        },
                    },
                ],
            })
            .unwrap();

        _ = dev.create_pipeline_layout(&PipelineLayoutDescriptor {
            bind_group_layouts: &[&BindGroupLayout {}, &BindGroupLayout {}],
        });

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

        _ = dev.create_shader_module(&ShaderModuleDescriptor { code: &[] });

        _ = dev.create_compute_pipeline(&ComputePipelineDescriptor {
            layout: &PipelineLayout {},
            compute: ProgrammableStage {
                module: &ShaderModule {},
                entry_point: "main".to_string(),
                constants: vec![PipelineConstant {
                    id: 0,
                    value: PipelineConstantValue::Float32(-1.0),
                }],
            },
        });

        _ = dev.create_render_pipeline(&RenderPipelineDescriptor {
            layout: &PipelineLayout {},
            vertex: VertexState {
                vertex: ProgrammableStage {
                    module: &ShaderModule {},
                    entry_point: "main".to_string(),
                    constants: vec![],
                },
                buffers: vec![VertexBufferLayout {
                    array_stride: 0,
                    step_mode: VertexStepMode::Vertex,
                    attributes: vec![
                        VertexAttribute {
                            format: VertexFormat::Float32x3,
                            offset: 0,
                            shader_location: 0,
                        },
                        VertexAttribute {
                            format: VertexFormat::Float32x2,
                            offset: 12,
                            shader_location: 1,
                        },
                    ],
                }],
            },
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: CullMode::Back,
                unclipped_depth: true,
            },
            depth_stencil: DepthStencilState {
                format: TextureFormat::Depth24PlusStencil8,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
                stencil_front: StencilFaceState {
                    compare: CompareFunction::Greater,
                    fail_op: StencilOperation::Keep,
                    depth_fail_op: StencilOperation::DecrementClamp,
                    pass_op: StencilOperation::Replace,
                },
                stencil_back: StencilFaceState {
                    compare: CompareFunction::Equal,
                    fail_op: StencilOperation::Zero,
                    depth_fail_op: StencilOperation::Invert,
                    pass_op: StencilOperation::IncrementWrap,
                },
                stencil_read_mask: 0xFF,
                stencil_write_mask: 0xFF,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            },
            multisample: MultisampleState {
                count: 1,
                mask: 0x1,
                alpha_to_coverage_enabled: false,
            },
            fragment: FragmentState {
                fragment: ProgrammableStage {
                    module: &ShaderModule {},
                    entry_point: "main".to_string(),
                    constants: vec![],
                },
                targets: vec![ColorTargetState {
                    format: TextureFormat::Rgba8Unorm,
                    blend: BlendState {
                        color: BlendComponent {
                            operation: BlendOperation::Add,
                            src_factor: BlendFactor::One,
                            dst_factor: BlendFactor::Zero,
                        },
                        alpha: BlendComponent {
                            operation: BlendOperation::Add,
                            src_factor: BlendFactor::One,
                            dst_factor: BlendFactor::Zero,
                        },
                    },
                    write_mask: ColorWrite::All.into(),
                }],
            },
        });

        _ = dev.create_command_encoder(None);

        _ = dev.create_render_bundle_encoder(&RenderBundleEncoderDescriptor {
            layout: RenderPassLayout {
                color_formats: vec![TextureFormat::Rgba8Unorm, TextureFormat::Rg11b10Ufloat],
                depth_stencil_format: Some(TextureFormat::Depth32Float),
                sample_count: 1,
            },
            depth_read_only: false,
            stencil_read_only: true,
        });

        _ = dev.create_query_set(&QuerySetDescriptor {
            kind: QueryKind::Occlusion,
            count: 16,
        });
    }

    #[test]
    fn queue() {
        let queue = Queue {};
        _ = queue.submit(Box::new([CommandBuffer {}]));
        _ = queue.on_submitted_work_done();
        _ = queue.write_buffer(&Buffer {}, 1024, &[1, 2, 3, 4]);
        _ = queue.write_texture(
            &ImageCopyTexture {
                texture: &Texture {},
                level: 0,
                origin: Origin3d { x: 0, y: 0, z: 0 },
                aspect: TextureAspect::All,
            },
            &[255u8; 16 * 16 * 1],
            ImageDataLayout {
                offset: 0,
                bytes_per_row: 16,
                rows_per_image: 4,
            },
            Extent3d {
                width: 4,
                height: 4,
                depth_or_layers: 1,
            },
        );
    }

    #[test]
    fn buffer() {
        let mut buf = Buffer {};
        _ = buf.size();
        _ = buf.usage();
        _ = buf.map_state();
        _ = buf.map(MapMode::Read, ..);
        _ = buf.get_mapped_range(256..512);
        buf.unmap();
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

    #[test]
    fn pipeline() {
        let comp = ComputePipeline {};
        let rend = RenderPipeline {};
        _ = comp.get_bind_group_layout(0);
        _ = rend.get_bind_group_layout(1);
    }

    #[test]
    fn command() {
        let mut enc = CommandEncoder {};

        let mut pass = enc.begin_compute_pass(&ComputePassDescriptor {
            timestamp_writes: vec![],
        });
        pass.set_bind_group(0, Some(&BindGroup {}), &[]);
        pass.set_pipeline(&ComputePipeline {});
        pass.dispatch_workgroups(32, 32, 1);
        pass.dispatch_workgroups_indirect(&Buffer {}, 0);
        _ = pass.end();

        let mut pass = enc.begin_render_pass(&RenderPassDescriptor {
            // TODO: Many things here should be optional.
            color_attachments: vec![RenderPassColorAttachment {
                view: &TextureView {},
                resolve_target: None,
                clear_value: Color::Float(0.0, 0.0, 0.0, 1.0),
                load_op: LoadOp::Load,
                store_op: StoreOp::Store,
            }],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: &TextureView {},
                depth_clear_value: 1.0,
                depth_load_op: LoadOp::Clear,
                depth_store_op: StoreOp::Store,
                depth_read_only: false,
                stencil_clear_value: 128,
                stencil_load_op: LoadOp::Clear,
                stencil_store_op: StoreOp::Discard,
                stencil_read_only: false,
            }),
            occlusion_query_set: &QuerySet {},
            timestamp_writes: RenderPassTimestampWrites {
                query_set: &QuerySet {},
                beginning_of_pass_write_index: 0,
                end_of_pass_write_index: 64,
            },
            max_draw_count: 1 << 20,
        });
        pass.set_viewport(0.0, 0.0, 480.0, 270.0, 0.0, 1.0);
        pass.set_scissor_rect(0, 0, 480, 270);
        pass.set_blend_constant(Color::Float(1.0, 1.0, 1.0, 1.0));
        pass.set_stencil_reference(0xFF);
        pass.begin_occlusion_query(0);
        pass.end_occlusion_query();
        pass.execute_bundles(&[&RenderBundle {}]);
        pass.set_bind_group(3, Some(&BindGroup {}), &[0, 256, 512]);
        pass.set_bind_group(1, None, &[]);
        pass.set_pipeline(&RenderPipeline {});
        pass.set_index_buffer(&Buffer {}, IndexFormat::Uint16, ..600);
        pass.set_vertex_buffer(0, &Buffer {}, 1024..1264);
        pass.set_vertex_buffer(1, &Buffer {}, 1048576..);
        pass.draw(36, 1, 0, 0);
        pass.draw_indexed(24, 1, 0, -2, 0);
        pass.draw_indirect(&Buffer {}, 0);
        pass.draw_indexed_indirect(&Buffer {}, 1 << 24);
        _ = pass.end();

        enc.copy_buffer_to_buffer(&Buffer {}, 0, &Buffer {}, 0, 4096);
        enc.copy_buffer_to_texture(
            &ImageCopyBuffer {
                buffer: &Buffer {},
                data_layout: ImageDataLayout {
                    offset: 0,
                    bytes_per_row: 4 * 1024,
                    rows_per_image: 1024,
                },
            },
            &ImageCopyTexture {
                texture: &Texture {},
                level: 0,
                origin: Origin3d { x: 0, y: 0, z: 0 },
                aspect: TextureAspect::All,
            },
            Extent3d {
                width: 1024,
                height: 1024,
                depth_or_layers: 3,
            },
        );
        enc.copy_texture_to_buffer(
            &ImageCopyTexture {
                texture: &Texture {},
                level: 1,
                origin: Origin3d { x: 0, y: 0, z: 0 },
                aspect: TextureAspect::All,
            },
            &ImageCopyBuffer {
                buffer: &Buffer {},
                data_layout: ImageDataLayout {
                    offset: 0,
                    bytes_per_row: 4 * 512,
                    rows_per_image: 512,
                },
            },
            Extent3d {
                width: 512,
                height: 512,
                depth_or_layers: 1,
            },
        );
        enc.copy_texture_to_texture(
            &ImageCopyTexture {
                texture: &Texture {},
                level: 0,
                origin: Origin3d { x: 0, y: 0, z: 0 },
                aspect: TextureAspect::All,
            },
            &ImageCopyTexture {
                texture: &Texture {},
                level: 0,
                origin: Origin3d { x: 256, y: 0, z: 0 },
                aspect: TextureAspect::All,
            },
            Extent3d {
                width: 256,
                height: 256,
                depth_or_layers: 1,
            },
        );
        enc.clear_buffer(&Buffer {}, ..);
        enc.write_timestamp(&QuerySet {}, 5);
        enc.resolve_query_set(&QuerySet {}, 1..10, &Buffer {}, 8192);
        _ = enc.finish(None);

        let mut enc = RenderBundleEncoder {};
        enc.set_bind_group(0, Some(&BindGroup {}), &[]);
        enc.set_pipeline(&RenderPipeline {});
        enc.set_index_buffer(&Buffer {}, IndexFormat::Uint32, ..280_000);
        enc.set_vertex_buffer(0, &Buffer {}, ..);
        enc.draw(3, 100, 0, 0);
        enc.draw_indexed(70_000, 1, 0, 0, 0);
        enc.draw_indirect(&Buffer {}, 1 << 21);
        enc.draw_indexed_indirect(&Buffer {}, 0);
        _ = enc.finish(None);
    }
}
