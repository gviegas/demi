//! GPU buffer.

use std::ops::{BitOr, RangeBounds};

use crate::Result;

pub struct Buffer {
    size: u64,
    usage: BufferUsageFlags,
    // TODO
}

impl Buffer {
    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn usage(&self) -> BufferUsageFlags {
        self.usage
    }

    pub fn map_state(&self) -> BufferMapState {
        panic!("not yet implemented");
    }

    // async
    pub fn map(&mut self, _mode: MapMode, _range: impl RangeBounds<u64>) {
        panic!("not yet implemented");
    }

    pub fn get_mapped_range(&self, _range: impl RangeBounds<u64>) -> Result<MappedRange> {
        panic!("not yet implemented");
    }

    pub fn unmap(&mut self) {
        panic!("not yet implemented");
    }
}

pub struct BufferDescriptor {
    pub size: u64,
    pub usage: BufferUsageFlags,
    pub mapped_at_creation: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum BufferUsage {
    MapRead = 0x1,
    MapWrite = 0x2,
    CopySrc = 0x4,
    CopyDst = 0x8,
    Index = 0x10,
    Vertex = 0x20,
    Uniform = 0x40,
    Storage = 0x80,
    Indirect = 0x100,
    QueryResolve = 0x200,
}

impl BitOr for BufferUsage {
    type Output = BufferUsageFlags;

    fn bitor(self, rhs: Self) -> Self::Output {
        BufferUsageFlags(self as u16 | rhs as u16)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BufferUsageFlags(u16);

impl BufferUsageFlags {
    pub fn is_set(self, usage: BufferUsage) -> bool {
        self.0 & usage as u16 != 0
    }
}

impl BitOr<BufferUsage> for BufferUsageFlags {
    type Output = Self;

    fn bitor(self, rhs: BufferUsage) -> Self::Output {
        Self(self.0 | rhs as u16)
    }
}

impl From<BufferUsage> for BufferUsageFlags {
    fn from(value: BufferUsage) -> Self {
        Self(value as u16)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BufferMapState {
    Unmapped,
    Pending,
    Mapped,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MapMode {
    Read,
    Write,
    //ReadWrite,
}

// TODO
pub struct MappedRange<'a> {
    _buffer: &'a Buffer,
    // ...
}

impl MappedRange<'_> {
    pub fn get(&self) -> &[u8] {
        panic!("not yet implemented");
    }

    pub fn get_mut(&mut self) -> &mut [u8] {
        panic!("not yet implemented");
    }
}
