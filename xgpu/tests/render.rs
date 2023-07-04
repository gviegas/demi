use std::mem;
use std::slice;

use xgpu::{
    self, Adapter, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingResourceLayout,
    Buffer, BufferBindingKind, BufferDescriptor, BufferUsage, Color, ColorTargetState, ColorWrite,
    CommandBuffer, CompareFunction, CullMode, DepthStencilState, Device, Extent3d, FragmentState,
    FrontFace, ImageCopyBuffer, ImageCopyTexture, ImageDataLayout, LoadOp, MapMode,
    MultisampleState, PipelineLayout, PipelineLayoutDescriptor, PowerPreference, PrimitiveState,
    PrimitiveTopology, ProgrammableStage, RenderPassColorAttachment,
    RenderPassDepthStencilAttachment, RenderPassDescriptor, RenderPipeline,
    RenderPipelineDescriptor, RequestAdapterOptions, ShaderModule, ShaderModuleDescriptor,
    ShaderStage, StoreOp, Texture, TextureAspect, TextureDescriptor, TextureDimension,
    TextureFormat, TextureUsage, TextureView, TextureViewDescriptor, TextureViewDimension,
    VertexAttribute, VertexBufferLayout, VertexFormat, VertexState, VertexStepMode,
};

const COLOR_FORMAT: TextureFormat = TextureFormat::Rgba8Unorm;
const DEPTH_FORMAT: TextureFormat = TextureFormat::Depth24Plus;
const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;
const STAGING_BUF_SIZE: u64 = 4 * WIDTH as u64 * HEIGHT as u64;
const VERTEX_BUF_SIZE: u64 = 3 * ((3 + 4) * mem::size_of::<f32>()) as u64;
const UNIFORM_BUF_SIZE: u64 = 16 * mem::size_of::<f32>() as u64;

fn request_adapter() -> Adapter {
    xgpu::request_adapter(Some(&RequestAdapterOptions {
        power_preference: PowerPreference::HighPerformance,
        force_fallback_adapter: false,
    }))
    .unwrap()
}

fn request_device(adapter: Adapter) -> Device {
    // TODO: Should be asynchronous.
    adapter.request_device(&Default::default()).unwrap()
}

fn create_staging_buffer(device: &mut Device) -> Buffer {
    device
        .create_buffer(&BufferDescriptor {
            size: STAGING_BUF_SIZE,
            usage: BufferUsage::MapRead | BufferUsage::CopyDst,
            mapped_at_creation: false,
        })
        .unwrap()
}

fn create_vertex_buffer(device: &mut Device) -> Buffer {
    device
        .create_buffer(&BufferDescriptor {
            size: VERTEX_BUF_SIZE,
            usage: BufferUsage::CopyDst | BufferUsage::Vertex,
            mapped_at_creation: false,
        })
        .unwrap()
}

fn create_uniform_buffer(device: &mut Device) -> Buffer {
    device
        .create_buffer(&BufferDescriptor {
            size: UNIFORM_BUF_SIZE,
            usage: BufferUsage::CopyDst | BufferUsage::Uniform,
            mapped_at_creation: false,
        })
        .unwrap()
}

fn create_color_texture(device: &mut Device) -> Texture {
    device
        .create_texture(&TextureDescriptor {
            size: Extent3d {
                width: WIDTH,
                height: HEIGHT,
                depth_or_layers: 1,
            },
            level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::Two,
            format: COLOR_FORMAT,
            usage: TextureUsage::CopySrc | TextureUsage::RenderAttachment,
            view_formats: &[],
        })
        .unwrap()
}

fn create_depth_texture(device: &mut Device) -> Texture {
    device
        .create_texture(&TextureDescriptor {
            size: Extent3d {
                width: WIDTH,
                height: HEIGHT,
                depth_or_layers: 1,
            },
            level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::Two,
            format: DEPTH_FORMAT,
            usage: TextureUsage::RenderAttachment.into(),
            view_formats: &[],
        })
        .unwrap()
}

fn create_color_or_depth_view(texture: &mut Texture) -> TextureView {
    texture
        .create_view(&TextureViewDescriptor {
            format: texture.format(),
            dimension: TextureViewDimension::Two,
            aspect: TextureAspect::All,
            level_range: ..1,
            layer_range: ..1,
        })
        .unwrap()
}

fn create_bind_group_layout(device: &mut Device) -> BindGroupLayout {
    device
        .create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStage::Vertex.into(),
                resource: BindingResourceLayout::Buffer {
                    kind: BufferBindingKind::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: UNIFORM_BUF_SIZE,
                },
            }],
        })
        .unwrap()
}

fn create_bind_group(device: &mut Device, layout: &BindGroupLayout, buffer: &Buffer) -> BindGroup {
    device
        .create_bind_group(&BindGroupDescriptor {
            layout: layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer {
                    buffer,
                    range: 0..UNIFORM_BUF_SIZE,
                },
            }],
        })
        .unwrap()
}

fn create_pipeline_layout(device: &mut Device, bg_layout: &BindGroupLayout) -> PipelineLayout {
    device
        .create_pipeline_layout(&PipelineLayoutDescriptor {
            bind_group_layouts: &[bg_layout],
        })
        .unwrap()
}

fn create_shader_module(device: &mut Device, code: &[u8]) -> ShaderModule {
    device
        .create_shader_module(&ShaderModuleDescriptor { code })
        .unwrap()
}

fn create_render_pipeline(
    device: &mut Device,
    layout: &PipelineLayout,
    vertex_module: &ShaderModule,
    fragment_module: &ShaderModule,
) -> RenderPipeline {
    let vertex = VertexState {
        vertex: ProgrammableStage {
            module: vertex_module,
            entry_point: "main".to_string(),
            constants: vec![],
        },
        buffers: vec![VertexBufferLayout {
            array_stride: (3 + 4) * mem::size_of::<f32>() as u64,
            step_mode: VertexStepMode::Vertex,
            attributes: vec![
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 3 * mem::size_of::<f32>() as u64,
                    shader_location: 1,
                },
            ],
        }],
    };

    let primitive = PrimitiveState {
        topology: PrimitiveTopology::TriangleList,
        strip_index_format: None,
        front_face: FrontFace::Ccw,
        cull_mode: CullMode::None,
        unclipped_depth: false,
    };

    let depth_stencil = DepthStencilState {
        format: DEPTH_FORMAT,
        depth_write_enabled: false,
        depth_compare: CompareFunction::Always,
        // TODO: Add a `new` function that takes only the previous
        // fields as parameters and defaults the rest.
        stencil_front: Default::default(),
        stencil_back: Default::default(),
        stencil_read_mask: !0,
        stencil_write_mask: !0,
        depth_bias: 0,
        depth_bias_slope_scale: 0.0,
        depth_bias_clamp: 0.0,
    };

    let multisample = MultisampleState::default();

    let fragment = FragmentState {
        fragment: ProgrammableStage {
            module: fragment_module,
            entry_point: "main".to_string(),
            constants: vec![],
        },
        targets: vec![Some(ColorTargetState {
            format: COLOR_FORMAT,
            blend: Default::default(),
            write_mask: ColorWrite::All.into(),
        })],
    };

    device
        .create_render_pipeline(&RenderPipelineDescriptor {
            layout,
            vertex,
            primitive,
            depth_stencil,
            multisample,
            fragment,
        })
        .unwrap()
}

struct Context {
    device: Device,
    staging_buf: Buffer,
    vertex_buf: Buffer,
    uniform_buf: Buffer,
    color_tex: Texture,
    _depth_tex: Texture,
    color_view: TextureView,
    depth_view: TextureView,
    _bg_layout: BindGroupLayout,
    bind_group: BindGroup,
    _pl_layout: PipelineLayout,
    _vs_mod: ShaderModule,
    _fs_mod: ShaderModule,
    pipeline: RenderPipeline,
}

impl Context {
    fn new() -> Self {
        let adapter = request_adapter();

        // This call invalidates `adapter`.
        let mut device = request_device(adapter);

        let staging_buf = create_staging_buffer(&mut device);
        let vertex_buf = create_vertex_buffer(&mut device);
        let uniform_buf = create_uniform_buffer(&mut device);

        let mut color_tex = create_color_texture(&mut device);
        let mut _depth_tex = create_depth_texture(&mut device);
        let color_view = create_color_or_depth_view(&mut color_tex);
        let depth_view = create_color_or_depth_view(&mut _depth_tex);

        let _bg_layout = create_bind_group_layout(&mut device);
        let bind_group = create_bind_group(&mut device, &_bg_layout, &uniform_buf);
        let _pl_layout = create_pipeline_layout(&mut device, &_bg_layout);

        // TODO: Code source.
        let _vs_mod = create_shader_module(&mut device, &[]);
        let _fs_mod = create_shader_module(&mut device, &[]);

        let pipeline = create_render_pipeline(&mut device, &_pl_layout, &_vs_mod, &_fs_mod);

        Self {
            device,
            staging_buf,
            vertex_buf,
            uniform_buf,
            color_tex,
            _depth_tex,
            color_view,
            depth_view,
            _bg_layout,
            bind_group,
            _pl_layout,
            _vs_mod,
            _fs_mod,
            pipeline,
        }
    }

    fn write_vertex_data(&mut self, data: &[f32; 3 * (3 + 4)]) {
        let len = mem::size_of_val(data);
        assert_eq!(len as u64, VERTEX_BUF_SIZE);
        let bytes = unsafe { slice::from_raw_parts(data.as_ptr().cast::<u8>(), len) };
        self.device
            .queue()
            .write_buffer(&self.vertex_buf, 0, bytes)
            .unwrap();
    }

    fn write_uniform_data(&mut self, data: &[f32; 16]) {
        let len = mem::size_of_val(data);
        assert_eq!(len as u64, UNIFORM_BUF_SIZE);
        let bytes = unsafe { slice::from_raw_parts(data.as_ptr().cast::<u8>(), len) };
        self.device
            .queue()
            .write_buffer(&self.uniform_buf, 0, bytes)
            .unwrap();
    }

    fn encode(&mut self) -> CommandBuffer {
        let mut enc = self.device.create_command_encoder(None).unwrap();

        let mut pass = enc.begin_render_pass(&RenderPassDescriptor {
            color_attachments: vec![Some(RenderPassColorAttachment {
                view: &self.color_view,
                resolve_target: None,
                clear_value: Some(Color::Float(0.1, 0.1, 0.1, 1.0)),
                load_op: LoadOp::Clear,
                store_op: StoreOp::Store,
            })],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: &self.depth_view,
                depth_clear_value: 0.0,
                depth_load_op: LoadOp::Clear,
                depth_store_op: StoreOp::Discard,
                depth_read_only: true,
                stencil_clear_value: 0,
                stencil_load_op: LoadOp::Clear,
                stencil_store_op: StoreOp::Discard,
                stencil_read_only: true,
            }),
            occlusion_query_set: None,
            timestamp_writes: None,
            max_draw_count: None, //1,
        });
        pass.set_viewport(0.0, 0.0, WIDTH as _, HEIGHT as _, 0.0, 1.0);
        pass.set_scissor_rect(0, 0, WIDTH, HEIGHT);
        pass.set_bind_group(0, Some(&self.bind_group), &[]);
        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, &self.vertex_buf, ..);
        pass.draw(3, 1, 0, 0);
        pass.end();

        // The implementation must ensure that this copy happens after
        // the render pass finishes writing to the color attachment
        // (synchronization is implicit).
        enc.copy_texture_to_buffer(
            &ImageCopyTexture {
                texture: &self.color_tex,
                level: 0,
                origin: Default::default(),
                aspect: TextureAspect::All,
                // ...
            },
            &ImageCopyBuffer {
                buffer: &self.staging_buf,
                data_layout: ImageDataLayout {
                    offset: 0,
                    bytes_per_row: WIDTH * 4,
                    rows_per_image: HEIGHT,
                },
            },
            Extent3d {
                width: WIDTH,
                height: HEIGHT,
                depth_or_layers: 1,
            },
        );

        enc.finish(None).unwrap()
    }

    fn submit_and_wait(&mut self, command_buffers: Box<[CommandBuffer]>) {
        self.device.queue().submit(command_buffers).unwrap();
        // TODO: Should be asynchronous.
        self.device.queue().on_submitted_work_done();
    }

    fn read_staging_buffer(&mut self, dst: &mut [u8]) {
        assert_eq!(dst.len() as u64, STAGING_BUF_SIZE);
        // TODO: Should be asynchronous.
        self.staging_buf.map(MapMode::Read, ..);
        let mapped_range = self.staging_buf.get_mapped_range(..).unwrap();
        let slc = mapped_range.get();
        assert_eq!(slc.len() as u64, STAGING_BUF_SIZE);
        dst.copy_from_slice(slc);
        self.staging_buf.unmap();
    }
}

// Must match DirectX/Metal coordinate system.
const VERTICES: [f32; 21] = [
    -1.0, -1.0, 0.5, // Position #0 - should be at bottom-left
    1.0, 0.0, 0.0, 1.0, // Color #0
    1.0, -1.0, 0.5, // Position #1 - should be at bottom-right
    0.0, 1.0, 0.0, 1.0, // Color #1
    0.0, 1.0, 0.5, // Position #2 - should be at top-center
    0.0, 0.0, 1.0, 1.0, // Color #2
];

// TODO: Projection.
const UNIFORM: [f32; 16] = [
    0.9, 0.0, 0.0, 0.0, //
    0.0, 0.9, 0.0, 0.0, //
    0.0, 0.0, 0.9, 0.0, //
    0.0, 0.0, 0.0, 1.0, //
];

#[test]
fn render() {
    let mut cx = Context::new();
    cx.write_vertex_data(&VERTICES);
    cx.write_uniform_data(&UNIFORM);
    let cmd_buf = cx.encode();
    cx.submit_and_wait(Box::new([cmd_buf]));
    let mut dst = Vec::with_capacity(STAGING_BUF_SIZE as _);
    cx.read_staging_buffer(&mut dst);
    // TODO: Validate result.
}
