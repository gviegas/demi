//! GPU queue.

use crate::{Buffer, CommandBuffer, Extent3d, Origin3d, Result, Texture, TextureAspect};

pub struct Queue {
    // TODO
}

// TODO: Cannot use mutable references here.
impl Queue {
    pub fn submit(&self, _command_buffers: Box<[CommandBuffer]>) -> Result<()> {
        panic!("not yet implemented");
    }

    // async
    pub fn on_submitted_work_done(&self) {
        panic!("not yet implemented");
    }

    pub fn write_buffer(&self, _buffer: &Buffer, _buffer_offset: u64, _data: &[u8]) -> Result<()> {
        panic!("not yet implemented");
    }

    pub fn write_texture(
        &self,
        _dst: &ImageCopyTexture,
        _data: &[u8],
        _data_layout: ImageDataLayout,
        _size: Extent3d,
    ) -> Result<()> {
        panic!("not yet implemented");
    }
}

pub struct QueueDescriptor;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ImageDataLayout {
    pub offset: u64,
    pub bytes_per_row: u32,
    pub rows_per_image: u32,
}

#[derive(Clone)]
pub struct ImageCopyTexture<'a> {
    pub texture: &'a Texture,
    pub level: u32,
    pub origin: Origin3d,
    pub aspect: TextureAspect,
}

#[derive(Clone)]
pub struct ImageCopyBuffer<'a> {
    pub buffer: &'a Buffer,
    pub data_layout: ImageDataLayout,
}
