//! GPU queue.

use crate::{Buffer, CommandBuffer, Extent3d, Origin3d, Result, Texture, TextureAspect};

pub struct Queue {
    // TODO
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        BufferDescriptor, BufferUsage, Extent3d, TextureDescriptor, TextureDimension,
        TextureFormat, TextureUsage,
    };

    #[test]
    fn queue() {
        let dev = crate::request_adapter(None)
            .unwrap()
            .request_device(None)
            .unwrap();
        let buf = dev
            .create_buffer(&BufferDescriptor {
                size: 4096,
                usage: BufferUsage::CopyDst | BufferUsage::Indirect,
                mapped_at_creation: false,
            })
            .unwrap();
        let tex = dev
            .create_texture(&TextureDescriptor {
                size: Extent3d {
                    width: 4,
                    height: 4,
                    depth_or_layers: 1,
                },
                level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::Two,
                format: TextureFormat::Rgba8Unorm,
                usage: TextureUsage::CopyDst | TextureUsage::TextureBinding,
                view_formats: &[],
            })
            .unwrap();

        // TODO: `Queue::new`.
        let queue = Queue {};
        _ = queue.submit(Box::new([CommandBuffer {}]));
        _ = queue.on_submitted_work_done();
        _ = queue.write_buffer(&buf, 1024, &[1, 2, 3, 4]);
        _ = queue.write_texture(
            &ImageCopyTexture {
                texture: &tex,
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
}
