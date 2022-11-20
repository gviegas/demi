// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use std::io::{self, Read};

/// Mesh.
pub struct Mesh {
    // TODO
}

/// Mesh primitive.
pub struct Primitive {
    // TODO
}

/// Mesh attribute semantics.
pub enum Semantic {
    Position,
    Normal,
    Tangent,
    TexCoord0,
    TexCoord1,
    Color0,
    Joints0,
    Weights0,
}

/// Mesh attribute data types.
pub enum DataType {
    F32,
    F32x2,
    F32x3,
    F32x4,
    U32,
    U32x2,
    U32x3,
    U32x4,
    U16,
    U16x2,
    U16x3,
    U16x4,
    U8,
    U8x2,
    U8x3,
    U8x4,
}

/// Primitive assembly modes.
pub enum Mode {
    PointList,
    LineList,
    LineStrip,
    TriangleList,
    TriangleStrip,
    TriangleFan,
}

/// Mesh builder.
pub struct Builder {
    // TODO
}

#[allow(unused_variables)] // TODO
#[allow(unused_mut)] // TODO
impl Builder {
    pub fn new() -> Self {
        todo!();
    }

    pub fn set_vertex_count(&mut self, count: usize) -> &mut Self {
        todo!();
    }

    pub fn set_semantic(
        &mut self,
        semantic: Semantic,
        data_type: DataType,
        offset: usize,
        stride: usize,
    ) -> &mut Self {
        todo!();
    }

    pub fn read_semantic<T: Read>(
        &mut self,
        semantic: Semantic,
        mut reader: T,
    ) -> io::Result<&mut Self> {
        todo!();
    }

    pub fn read_vertices<T: Read>(&mut self, mut reader: T) -> io::Result<&mut Self> {
        todo!();
    }

    pub fn set_indexed(&mut self, count: usize, data_type: DataType) -> &mut Self {
        todo!();
    }

    pub fn read_indices<T: Read>(&mut self, mut reader: T) -> io::Result<&mut Self> {
        todo!();
    }

    pub fn push_primitive(&mut self, mode: Mode) -> io::Result<&mut Self> {
        todo!();
    }

    pub fn create(&mut self) -> io::Result<Mesh> {
        todo!();
    }
}
