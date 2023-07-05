//! GPU command encoding.

use std::ops::RangeBounds;

use crate::{
    BindGroup, Buffer, ComputePipeline, Extent3d, ImageCopyBuffer, ImageCopyTexture, IndexFormat,
    QuerySet, RenderPipeline, Result, TextureFormat, TextureView,
};

pub struct CommandEncoder {
    // TODO
}

impl CommandEncoder {
    // TODO: Will need `Arc` or similar for these resources.

    pub fn begin_compute_pass(
        &mut self,
        _desc: Option<&ComputePassDescriptor>,
    ) -> ComputePassEncoder {
        panic!("not yet implemented");
    }

    pub fn begin_render_pass(&mut self, _desc: &RenderPassDescriptor) -> RenderPassEncoder {
        panic!("not yet implemented");
    }

    pub fn copy_buffer_to_buffer(
        &mut self,
        _src: &Buffer,
        _src_offset: u64,
        _dst: &Buffer,
        _dst_offset: u64,
        _size: u64,
    ) {
        panic!("not yet implemented");
    }

    pub fn copy_buffer_to_texture(
        &mut self,
        _src: &ImageCopyBuffer,
        _dst: &ImageCopyTexture,
        _copy_size: Extent3d,
    ) {
        panic!("not yet implemented");
    }

    pub fn copy_texture_to_buffer(
        &mut self,
        _src: &ImageCopyTexture,
        _dst: &ImageCopyBuffer,
        _copy_size: Extent3d,
    ) {
        panic!("not yet implemented");
    }

    pub fn copy_texture_to_texture(
        &mut self,
        _src: &ImageCopyTexture,
        _dst: &ImageCopyTexture,
        _copy_size: Extent3d,
    ) {
        panic!("not yet implemented");
    }

    pub fn clear_buffer(&mut self, _buffer: &Buffer, _range: impl RangeBounds<u64>) {
        panic!("not yet implemented");
    }

    pub fn write_timestamp(&mut self, _query_set: &QuerySet, _query_index: u32) {
        panic!("not yet implemented");
    }

    pub fn resolve_query_set(
        &mut self,
        _query_set: &QuerySet,
        _query_range: impl RangeBounds<u32>,
        _dst: &Buffer,
        _dst_offset: u64,
    ) {
        panic!("not yet implemented");
    }

    pub fn finish(self, _desc: Option<&CommandBufferDescriptor>) -> Result<CommandBuffer> {
        panic!("not yet implemented");
    }

    // Debug markers.

    pub fn push_debug_group(&mut self, _group_label: String) {
        panic!("not yet implemented");
    }

    pub fn pop_debug_group(&mut self) {
        panic!("not yet implemented");
    }

    pub fn insert_debug_marker(&mut self, _marker_label: String) {
        panic!("not yet implemented");
    }
}

pub struct CommandEncoderDescriptor;

pub struct ComputePassEncoder {
    // TODO
}

impl ComputePassEncoder {
    pub fn set_bind_group(
        &mut self,
        _index: u32,
        _bind_group: Option<&BindGroup>,
        _dynamic_offsets: &[u32],
    ) {
        panic!("not yet implemented");
    }

    pub fn set_pipeline(&mut self, _pipeline: &ComputePipeline) {
        panic!("not yet implemented");
    }

    pub fn dispatch_workgroups(
        &mut self,
        _workgroup_count_x: u32,
        _workgroup_count_y: u32,
        _workgroup_count_z: u32,
    ) {
        panic!("not yet implemented");
    }

    pub fn dispatch_workgroups_indirect(
        &mut self,
        _indirect_buffer: &Buffer,
        _indirect_offset: u64,
    ) {
        panic!("not yet implemented");
    }

    pub fn end(self) {
        panic!("not yet implemented");
    }

    // Debug markers.

    pub fn push_debug_group(&mut self, _group_label: String) {
        panic!("not yet implemented");
    }

    pub fn pop_debug_group(&mut self) {
        panic!("not yet implemented");
    }

    pub fn insert_debug_marker(&mut self, _marker_label: String) {
        panic!("not yet implemented");
    }
}

pub struct ComputePassDescriptor<'a> {
    pub timestamp_writes: Vec<ComputePassTimestampWrites<'a>>,
}

pub struct ComputePassTimestampWrites<'a> {
    pub query_set: &'a QuerySet,
    pub beginning_of_pass_write_index: u32,
    pub end_of_pass_write_index: u32,
}

pub struct RenderPassEncoder {
    // TODO
}

impl RenderPassEncoder {
    pub fn set_viewport(
        &mut self,
        _x: f32,
        _y: f32,
        _width: f32,
        _height: f32,
        _min_depth: f32,
        _max_depth: f32,
    ) {
        panic!("not yet implemented");
    }

    pub fn set_scissor_rect(&mut self, _x: u32, _y: u32, _width: u32, _height: u32) {
        panic!("not yet implemented");
    }

    pub fn set_blend_constant(&mut self, _color: Color) {
        panic!("not yet implemented");
    }

    pub fn set_stencil_reference(&mut self, _reference: u32) {
        panic!("not yet implemented");
    }

    pub fn begin_occlusion_query(&mut self, _query_index: u32) {
        panic!("not yet implemented");
    }

    pub fn end_occlusion_query(&mut self) {
        panic!("not yet implemented");
    }

    pub fn execute_bundles(&mut self, _bundles: &[&RenderBundle]) {
        panic!("not yet implemented");
    }

    pub fn set_bind_group(
        &mut self,
        _index: u32,
        _bind_group: Option<&BindGroup>,
        _dynamic_offsets: &[u32],
    ) {
        panic!("not yet implemented");
    }

    pub fn set_pipeline(&mut self, _pipeline: &RenderPipeline) {
        panic!("not yet implemented");
    }

    pub fn set_index_buffer(
        &mut self,
        _buffer: &Buffer,
        _index_format: IndexFormat,
        _range: impl RangeBounds<u64>,
    ) {
        panic!("not yet implemented");
    }

    pub fn set_vertex_buffer(
        &mut self,
        _slot: u32,
        _buffer: &Buffer,
        _range: impl RangeBounds<u64>,
    ) {
        panic!("not yet implemented");
    }

    pub fn draw(
        &mut self,
        _vertex_count: u32,
        _instance_count: u32,
        _first_vertex: u32,
        _first_instance: u32,
    ) {
        panic!("not yet implemented");
    }

    pub fn draw_indexed(
        &mut self,
        _index_count: u32,
        _instance_count: u32,
        _first_index: u32,
        _base_vertex: i32,
        _first_instance: u32,
    ) {
        panic!("not yet implemented");
    }

    pub fn draw_indirect(&mut self, _indirect_buffer: &Buffer, _indirect_offset: u64) {
        panic!("not yet implemented");
    }

    pub fn draw_indexed_indirect(&mut self, _indirect_buffer: &Buffer, _indirect_offset: u64) {
        panic!("not yet implemented");
    }

    pub fn end(self) {
        panic!("not yet implemented");
    }

    // Debug markers.

    pub fn push_debug_group(&mut self, _group_label: String) {
        panic!("not yet implemented");
    }

    pub fn pop_debug_group(&mut self) {
        panic!("not yet implemented");
    }

    pub fn insert_debug_marker(&mut self, _marker_label: String) {
        panic!("not yet implemented");
    }
}

pub struct RenderPassDescriptor<'a, 'b, 'c, 'd, 'e> {
    pub color_attachments: Vec<Option<RenderPassColorAttachment<'a, 'b>>>,
    pub depth_stencil_attachment: Option<RenderPassDepthStencilAttachment<'c>>,
    pub occlusion_query_set: Option<&'d QuerySet>,
    pub timestamp_writes: Option<RenderPassTimestampWrites<'e>>,
    pub max_draw_count: Option<u64>,
}

pub struct RenderPassColorAttachment<'a, 'b> {
    pub view: &'a TextureView,
    pub resolve_target: Option<&'b TextureView>,
    pub clear_value: Option<Color>,
    pub load_op: LoadOp,
    pub store_op: StoreOp,
}

pub struct RenderPassDepthStencilAttachment<'a> {
    pub view: &'a TextureView,
    pub depth_clear_value: f32,
    pub depth_load_op: LoadOp,
    pub depth_store_op: StoreOp,
    pub depth_read_only: bool,
    pub stencil_clear_value: u32,
    pub stencil_load_op: LoadOp,
    pub stencil_store_op: StoreOp,
    pub stencil_read_only: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LoadOp {
    Load,
    Clear,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StoreOp {
    Store,
    Discard,
}

pub struct RenderPassTimestampWrites<'a> {
    pub query_set: &'a QuerySet,
    pub beginning_of_pass_write_index: u32,
    pub end_of_pass_write_index: u32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    Float(f32, f32, f32, f32),
    Sint(i32, i32, i32, i32),
    Uint(u32, u32, u32, u32),
}

pub struct CommandBuffer {
    // TODO
}

pub struct CommandBufferDescriptor;

pub struct RenderBundleEncoder {
    // TODO
}

impl RenderBundleEncoder {
    pub fn set_bind_group(
        &mut self,
        _index: u32,
        _bind_group: Option<&BindGroup>,
        _dynamic_offsets: &[u32],
    ) {
        panic!("not yet implemented");
    }

    pub fn set_pipeline(&mut self, _pipeline: &RenderPipeline) {
        panic!("not yet implemented");
    }

    pub fn set_index_buffer(
        &mut self,
        _buffer: &Buffer,
        _index_format: IndexFormat,
        _range: impl RangeBounds<u64>,
    ) {
        panic!("not yet implemented");
    }

    pub fn set_vertex_buffer(
        &mut self,
        _slot: u32,
        _buffer: &Buffer,
        _range: impl RangeBounds<u64>,
    ) {
        panic!("not yet implemented");
    }

    pub fn draw(
        &mut self,
        _vertex_count: u32,
        _instance_count: u32,
        _first_vertex: u32,
        _first_instance: u32,
    ) {
        panic!("not yet implemented");
    }

    pub fn draw_indexed(
        &mut self,
        _index_count: u32,
        _instance_count: u32,
        _first_index: u32,
        _base_vertex: i32,
        _first_instance: u32,
    ) {
        panic!("not yet implemented");
    }

    pub fn draw_indirect(&mut self, _indirect_buffer: &Buffer, _indirect_offset: u64) {
        panic!("not yet implemented");
    }

    pub fn draw_indexed_indirect(&mut self, _indirect_buffer: &Buffer, _indirect_offset: u64) {
        panic!("not yet implemented");
    }

    pub fn finish(self, _desc: Option<&RenderBundleDescriptor>) -> Result<RenderBundle> {
        panic!("not yet implemented");
    }

    // Debug markers.

    pub fn push_debug_group(&mut self, _group_label: String) {
        panic!("not yet implemented");
    }

    pub fn pop_debug_group(&mut self) {
        panic!("not yet implemented");
    }

    pub fn insert_debug_marker(&mut self, _marker_label: String) {
        panic!("not yet implemented");
    }
}

pub struct RenderBundleEncoderDescriptor {
    pub layout: RenderPassLayout,
    pub depth_read_only: bool,
    pub stencil_read_only: bool,
}

pub struct RenderPassLayout {
    pub color_formats: Vec<Option<TextureFormat>>,
    pub depth_stencil_format: Option<TextureFormat>,
    pub sample_count: u32,
}

pub struct RenderBundle {
    // TODO
}

pub struct RenderBundleDescriptor;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        BindGroup, Buffer, BufferDescriptor, BufferUsage, ComputePipeline, Extent3d,
        ImageCopyBuffer, ImageCopyTexture, ImageDataLayout, IndexFormat, Origin3d, QueryKind,
        QuerySet, QuerySetDescriptor, RenderPipeline, Result, TextureAspect, TextureDescriptor,
        TextureDimension, TextureFormat, TextureUsage, TextureView, TextureViewDescriptor,
        TextureViewDimension,
    };

    #[test]
    fn command() {
        let mut dev = crate::request_adapter(None)
            .unwrap()
            .request_device(None)
            .unwrap();
        let rbuf = dev
            .create_buffer(&BufferDescriptor {
                size: 1 << 22,
                usage: BufferUsage::CopySrc
                    | BufferUsage::CopyDst
                    | BufferUsage::Index
                    | BufferUsage::Vertex
                    | BufferUsage::Uniform
                    | BufferUsage::Indirect,
                mapped_at_creation: false,
            })
            .unwrap();
        let wbuf = dev
            .create_buffer(&BufferDescriptor {
                size: 1 << 20,
                usage: BufferUsage::CopySrc
                    | BufferUsage::CopyDst
                    | BufferUsage::QueryResolve
                    | BufferUsage::Storage,
                mapped_at_creation: false,
            })
            .unwrap();
        let rtex = dev
            .create_texture(&TextureDescriptor {
                size: Extent3d {
                    width: 1024,
                    height: 1024,
                    depth_or_layers: 3,
                },
                level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::Two,
                format: TextureFormat::Rgba8Unorm,
                usage: TextureUsage::CopySrc | TextureUsage::CopyDst | TextureUsage::TextureBinding,
                view_formats: &[],
            })
            .unwrap();
        let wtex = dev
            .create_texture(&TextureDescriptor {
                size: Extent3d {
                    width: 256,
                    height: 256,
                    depth_or_layers: 1,
                },
                level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::Two,
                format: TextureFormat::Rgba8Unorm,
                usage: TextureUsage::CopySrc | TextureUsage::CopyDst | TextureUsage::StorageBinding,
                view_formats: &[],
            })
            .unwrap();
        let mut color_tex = dev
            .create_texture(&TextureDescriptor {
                size: Extent3d {
                    width: 480,
                    height: 270,
                    depth_or_layers: 1,
                },
                level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::Two,
                format: TextureFormat::Rgba8Unorm,
                usage: TextureUsage::RenderAttachment.into(),
                view_formats: &[],
            })
            .unwrap();
        let color_view = color_tex
            .create_view(&TextureViewDescriptor {
                format: color_tex.format(),
                dimension: TextureViewDimension::Two,
                aspect: TextureAspect::All,
                level_range: ..,
                layer_range: ..,
            })
            .unwrap();
        let mut ds_tex = dev
            .create_texture(&TextureDescriptor {
                size: Extent3d {
                    width: 480,
                    height: 270,
                    depth_or_layers: 1,
                },
                level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::Two,
                format: TextureFormat::Depth24PlusStencil8,
                usage: TextureUsage::RenderAttachment.into(),
                view_formats: &[],
            })
            .unwrap();
        let ds_view = ds_tex
            .create_view(&TextureViewDescriptor {
                format: color_tex.format(),
                dimension: TextureViewDimension::Two,
                aspect: TextureAspect::All,
                level_range: ..,
                layer_range: ..,
            })
            .unwrap();
        let occ_qs = dev
            .create_query_set(&QuerySetDescriptor {
                kind: QueryKind::Occlusion,
                count: 16,
            })
            .unwrap();
        let ts_qs = dev
            .create_query_set(&QuerySetDescriptor {
                kind: QueryKind::Timestamp,
                count: 32,
            })
            .unwrap();

        // TODO: `CommandEncoder::new`.
        let mut enc = CommandEncoder {};

        enc.push_debug_group("dbg1".to_string());
        let mut pass = enc.begin_compute_pass(Some(&ComputePassDescriptor {
            timestamp_writes: vec![],
        }));
        pass.set_bind_group(0, Some(&BindGroup {}), &[]);
        pass.set_pipeline(&ComputePipeline {});
        pass.dispatch_workgroups(32, 32, 1);
        pass.dispatch_workgroups_indirect(&rbuf, 0);
        pass.end();
        enc.pop_debug_group();

        enc.insert_debug_marker("dbg2".to_string());
        let mut pass = enc.begin_render_pass(&RenderPassDescriptor {
            color_attachments: vec![Some(RenderPassColorAttachment {
                view: &color_view,
                resolve_target: None,
                clear_value: Some(Color::Float(0.0, 0.0, 0.0, 1.0)),
                load_op: LoadOp::Load,
                store_op: StoreOp::Store,
            })],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: &ds_view,
                depth_clear_value: 1.0,
                depth_load_op: LoadOp::Clear,
                depth_store_op: StoreOp::Store,
                depth_read_only: false,
                stencil_clear_value: 128,
                stencil_load_op: LoadOp::Clear,
                stencil_store_op: StoreOp::Discard,
                stencil_read_only: false,
            }),
            occlusion_query_set: Some(&occ_qs),
            timestamp_writes: Some(RenderPassTimestampWrites {
                query_set: &ts_qs,
                beginning_of_pass_write_index: 0,
                end_of_pass_write_index: 1,
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
        pass.set_index_buffer(&rbuf, IndexFormat::Uint16, ..600);
        pass.set_vertex_buffer(0, &rbuf, 1024..1264);
        pass.set_vertex_buffer(1, &rbuf, 8192..8672);
        pass.draw(36, 1, 0, 0);
        pass.draw_indexed(24, 1, 0, -2, 0);
        pass.draw_indirect(&rbuf, 0);
        pass.draw_indexed_indirect(&rbuf, 1 << 20);
        pass.end();

        enc.copy_buffer_to_buffer(&rbuf, 0, &wbuf, 0, 4096);
        enc.copy_buffer_to_texture(
            &ImageCopyBuffer {
                buffer: &rbuf,
                data_layout: ImageDataLayout {
                    offset: 0,
                    bytes_per_row: 4 * 1024,
                    rows_per_image: 1024,
                },
            },
            &ImageCopyTexture {
                texture: &rtex,
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
                texture: &rtex,
                level: 1,
                origin: Origin3d::default(),
                aspect: TextureAspect::All,
            },
            &ImageCopyBuffer {
                buffer: &wbuf,
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
                texture: &rtex,
                level: 0,
                origin: Origin3d {
                    x: 512,
                    y: 512,
                    z: 0,
                },
                aspect: TextureAspect::All,
            },
            &ImageCopyTexture {
                texture: &wtex,
                level: 0,
                origin: Origin3d { x: 0, y: 0, z: 0 },
                aspect: TextureAspect::All,
            },
            Extent3d {
                width: 256,
                height: 256,
                depth_or_layers: 1,
            },
        );
        enc.clear_buffer(&wbuf, ..);
        enc.write_timestamp(&ts_qs, 5);
        enc.resolve_query_set(&occ_qs, ..1, &wbuf, 1024);
        _ = enc.finish(None);

        let mut enc = RenderBundleEncoder {};
        enc.insert_debug_marker("dbg3".to_string());
        enc.set_bind_group(0, Some(&BindGroup {}), &[]);
        enc.set_pipeline(&RenderPipeline {});
        enc.set_index_buffer(&rbuf, IndexFormat::Uint32, ..280_000);
        enc.set_vertex_buffer(0, &rbuf, ..);
        enc.draw(3, 100, 0, 0);
        enc.draw_indexed(70_000, 1, 0, 0, 0);
        enc.draw_indirect(&rbuf, 1 << 21);
        enc.draw_indexed_indirect(&rbuf, 0);
        _ = enc.finish(None);
    }
}
