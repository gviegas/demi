//! GPU command encoding.

use std::ops::RangeBounds;

use crate::{BindGroup, Buffer, ComputePipeline, Extent3d, QuerySet, Result};

pub struct CommandBuffer {
    // TODO
}

pub struct CommandBufferDescriptor;

pub struct CommandEncoder {
    // TODO
}

impl CommandEncoder {
    // TODO: Will need `Arc` or similar for these resources.

    pub fn begin_compute_pass(&mut self, _desc: &ComputePassDescriptor) -> ComputePassEncoder {
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

    // TODO: Debug markers.
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

    // TODO: Debug markers.
}

pub struct ComputePassDescriptor<'a> {
    pub timestamp_writes: Vec<ComputePassTimestampWrites<'a>>,
}

pub struct ComputePassTimestampWrites<'a> {
    pub query_set: &'a QuerySet,
    pub beginning_of_pass_write_index: u32,
    pub end_of_pass_write_index: u32,
}

// TODO
pub struct RenderPassEncoder;
pub struct RenderPassDescriptor;
pub struct ImageCopyBuffer;
pub struct ImageCopyTexture;
