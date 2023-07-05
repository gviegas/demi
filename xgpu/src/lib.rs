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
        let rng1 = buf.get_mapped_range(256..512).unwrap();
        let mut rng2 = buf.get_mapped_range(0..256).unwrap();
        _ = rng1.get();
        _ = rng2.get_mut();
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

        enc.push_debug_group("dbg1".to_string());
        let mut pass = enc.begin_compute_pass(Some(&ComputePassDescriptor {
            timestamp_writes: vec![],
        }));
        pass.set_bind_group(0, Some(&BindGroup {}), &[]);
        pass.set_pipeline(&ComputePipeline {});
        pass.dispatch_workgroups(32, 32, 1);
        pass.dispatch_workgroups_indirect(&Buffer {}, 0);
        pass.end();
        enc.pop_debug_group();

        enc.insert_debug_marker("dbg2".to_string());
        let mut pass = enc.begin_render_pass(&RenderPassDescriptor {
            color_attachments: vec![Some(RenderPassColorAttachment {
                view: &TextureView {},
                resolve_target: None,
                clear_value: Some(Color::Float(0.0, 0.0, 0.0, 1.0)),
                load_op: LoadOp::Load,
                store_op: StoreOp::Store,
            })],
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
            occlusion_query_set: Some(&QuerySet {}),
            timestamp_writes: Some(RenderPassTimestampWrites {
                query_set: &QuerySet {},
                beginning_of_pass_write_index: 0,
                end_of_pass_write_index: 64,
            }),
            max_draw_count: Some(1 << 20),
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
        pass.end();

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
                origin: Origin3d::default(),
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
                origin: Origin3d::default(),
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
        enc.insert_debug_marker("dbg3".to_string());
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
