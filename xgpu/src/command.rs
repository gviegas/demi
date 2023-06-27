//! GPU command encoding.

use std::ops::RangeBounds;

use crate::{Buffer, Extent3d, QuerySet, Result};

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
}

pub struct CommandEncoderDescriptor;

// TODO
pub struct ComputePassEncoder;
pub struct ComputePassDescriptor;
pub struct RenderPassEncoder;
pub struct RenderPassDescriptor;
pub struct ImageCopyBuffer;
pub struct ImageCopyTexture;
