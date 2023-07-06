//! GPU device.

use std::ops::RangeBounds;

use crate::{
    BindGroup, BindGroupDescriptor, BindGroupLayout, BindGroupLayoutDescriptor, Buffer,
    BufferDescriptor, CommandEncoder, CommandEncoderDescriptor, ComputePipeline,
    ComputePipelineDescriptor, Feature, Limit, PipelineLayout, PipelineLayoutDescriptor, QuerySet,
    QuerySetDescriptor, Queue, QueueDescriptor, RenderBundleEncoder, RenderBundleEncoderDescriptor,
    RenderPipeline, RenderPipelineDescriptor, Result, Sampler, SamplerDescriptor, ShaderModule,
    ShaderModuleDescriptor, SupportedFeatures, SupportedLimits, Texture, TextureDescriptor,
};

pub struct Device {
    // TODO: `adapter`.
    features: SupportedFeatures,
    limits: SupportedLimits,
    // TODO: `queue`.
    // TODO
}

impl Device {
    pub fn features(&self) -> &SupportedFeatures {
        &self.features
    }

    pub fn limits(&self) -> &SupportedLimits {
        &self.limits
    }

    pub fn queue(&self) -> &Queue {
        panic!("not yet implemented");
    }

    pub fn create_buffer(&self, _desc: &BufferDescriptor) -> Result<Buffer> {
        panic!("not yet implemented");
    }

    pub fn create_texture(&self, _desc: &TextureDescriptor) -> Result<Texture> {
        panic!("not yet implemented");
    }

    pub fn create_sampler<T>(&self, _desc: &SamplerDescriptor<T>) -> Result<Sampler>
    where
        T: RangeBounds<f32>,
    {
        panic!("not yet implemented");
    }

    pub fn create_bind_group_layout(
        &self,
        _desc: &BindGroupLayoutDescriptor,
    ) -> Result<BindGroupLayout> {
        panic!("not yet implemented");
    }

    pub fn create_pipeline_layout(
        &self,
        _desc: &PipelineLayoutDescriptor,
    ) -> Result<PipelineLayout> {
        panic!("not yet implemented");
    }

    pub fn create_bind_group(&self, _desc: &BindGroupDescriptor) -> Result<BindGroup> {
        panic!("not yet implemented");
    }

    pub fn create_shader_module(&self, _desc: &ShaderModuleDescriptor) -> Result<ShaderModule> {
        panic!("not yet implemented");
    }

    // async
    pub fn create_compute_pipeline(
        &self,
        _desc: &ComputePipelineDescriptor,
    ) -> Result<ComputePipeline> {
        panic!("not yet implemented");
    }

    // async
    pub fn create_render_pipeline(
        &self,
        _desc: &RenderPipelineDescriptor,
    ) -> Result<RenderPipeline> {
        panic!("not yet implemented");
    }

    pub fn create_command_encoder(
        &self,
        _desc: Option<&CommandEncoderDescriptor>,
    ) -> Result<CommandEncoder> {
        panic!("not yet implemented");
    }

    pub fn create_render_bundle_encoder(
        &self,
        _desc: &RenderBundleEncoderDescriptor,
    ) -> Result<RenderBundleEncoder> {
        panic!("not yet implemented");
    }

    pub fn create_query_set(&self, _desc: &QuerySetDescriptor) -> Result<QuerySet> {
        panic!("not yet implemented");
    }
}

pub struct DeviceDescriptor<'a, 'b> {
    pub required_features: &'a [Feature],
    pub required_limits: &'b [Limit],
    pub default_queue: QueueDescriptor,
}

impl Default for DeviceDescriptor<'_, '_> {
    fn default() -> Self {
        Self {
            required_features: &[],
            required_limits: &[],
            default_queue: QueueDescriptor {},
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        AddressMode, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, BindingResource,
        BindingResourceLayout, BlendComponent, BlendFactor, BlendOperation, BlendState,
        BufferBindingKind, BufferUsage, ColorTargetState, ColorWrite, CompareFunction, CullMode,
        DepthStencilState, Extent3d, FilterMode, FragmentState, FrontFace, MipmapFilterMode,
        MultisampleState, PipelineConstant, PipelineConstantValue, PrimitiveState,
        PrimitiveTopology, ProgrammableStage, QueryKind, RenderPassLayout, SamplerBindingKind,
        ShaderStage, StencilFaceState, StencilOperation, TextureAspect, TextureDimension,
        TextureFormat, TextureSampleKind, TextureUsage, TextureViewDescriptor,
        TextureViewDimension, VertexAttribute, VertexBufferLayout, VertexFormat, VertexState,
        VertexStepMode,
    };

    #[test]
    fn device() {
        // TODO: `Device::new`.
        let dev = Device {
            features: Default::default(), // NOTE: This is invalid.
            limits: Default::default(),
        };
        _ = dev.features();
        _ = dev.limits();
        _ = dev.queue();

        let buf = dev
            .create_buffer(&BufferDescriptor {
                size: 16384,
                usage: BufferUsage::CopyDst
                    | BufferUsage::QueryResolve
                    | BufferUsage::Storage
                    | BufferUsage::Uniform,
                mapped_at_creation: false,
            })
            .unwrap();

        let tex = dev
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
        let view = tex
            .create_view(&TextureViewDescriptor {
                format: tex.format(),
                dimension: TextureViewDimension::Two,
                aspect: TextureAspect::All,
                level_range: ..,
                layer_range: ..,
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
        let splr = dev.create_sampler(&Default::default()).unwrap();

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
                    resource: BindingResource::Texture(&view),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::Sampler(&splr),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: BindingResource::Buffer {
                        buffer: &buf,
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
                targets: vec![Some(ColorTargetState {
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
                })],
            },
        });

        _ = dev.create_command_encoder(None);

        _ = dev.create_render_bundle_encoder(&RenderBundleEncoderDescriptor {
            layout: RenderPassLayout {
                color_formats: vec![
                    Some(TextureFormat::Rgba8Unorm),
                    None,
                    Some(TextureFormat::Rg11b10Ufloat),
                    Some(TextureFormat::Bgra8UnormSrgb),
                ],
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
}
