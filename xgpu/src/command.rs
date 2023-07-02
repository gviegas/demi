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

    pub fn finish(&mut self, _desc: Option<&CommandBufferDescriptor>) -> Result<CommandBuffer> {
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

    pub fn end(self) -> Result<()> {
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

    pub fn end(self) -> Result<()> {
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
