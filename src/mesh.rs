// Copyright 2022 Gustavo C. Viegas. All rights reserved.

//! Geometry for drawing.

use std::io::{self, Read};
use std::ptr::NonNull;
use std::sync::{Arc, RwLock};

use crate::gpu::{self, BufId, BufOptions};
use crate::material::Material;
use crate::var_buf::{VarAlloc, VarBuf, VarEntry};

/// Vertex buffer's allocation.
#[derive(Debug)]
pub(crate) struct VertAlloc {
    ptr: NonNull<()>,
    size: usize,
    gid: Option<BufId>,
}

impl VertAlloc {
    /// Creates a new vertex buffer allocation.
    ///
    /// This functions will attempt to create an allocation of
    /// `size_hint` (plus alignment padding) bytes.
    /// It will halve this size until creation succeeds.
    ///
    /// Creating a zero-sized [`VertAlloc`] does not allocate
    /// [`gpu`] resources.
    pub fn new(size_hint: usize) -> Self {
        debug_assert_eq!(0, Self::MIN_ALIGN & Self::MIN_ALIGN - 1);
        let mut size = size_hint + VertAlloc::MIN_ALIGN - 1 & !(VertAlloc::MIN_ALIGN - 1);
        loop {
            if size > 0 {
                if let Ok(mut gid) = gpu::create_vb(&BufOptions {
                    size: size as u64,
                    cpu_visible: true,
                }) {
                    if let Ok(ptr) = gpu::buffer_ptr(&gid) {
                        break Self {
                            ptr,
                            size,
                            gid: Some(gid),
                        };
                    }
                    gpu::drop_buffer(&mut gid);
                }
                size /= 2;
            } else {
                break Self {
                    ptr: NonNull::dangling(),
                    size: 0,
                    gid: None,
                };
            }
        }
    }
}

impl VarAlloc for VertAlloc {
    fn grow(&mut self, new_size: usize) -> io::Result<NonNull<()>> {
        if self.size >= new_size {
            Ok(self.ptr)
        } else {
            // TODO: Provide a `gpu` function that
            // explicitly resizes a buffer, so it
            // can try to realloc/unmap memory.
            let gid = gpu::create_vb(&BufOptions {
                size: new_size as u64,
                cpu_visible: true,
            })?;
            match gpu::buffer_ptr(&gid) {
                Ok(ptr) => {
                    if let Some(ref mut x) = self.gid {
                        gpu::drop_buffer(x);
                    }
                    self.ptr = ptr;
                    self.size = new_size;
                    self.gid = Some(gid);
                    Ok(ptr)
                }
                Err(e) => Err(e),
            }
        }
    }

    fn shrink(&mut self, new_size: usize) -> io::Result<NonNull<()>> {
        if self.size <= new_size {
            Ok(self.ptr)
        } else if new_size == 0 {
            gpu::drop_buffer(self.gid.as_mut().unwrap());
            self.ptr = NonNull::dangling();
            self.size = 0;
            self.gid = None;
            Ok(self.ptr)
        } else {
            // TODO: See `grow` above.
            let gid = gpu::create_vb(&BufOptions {
                size: new_size as u64,
                cpu_visible: true,
            })?;
            match gpu::buffer_ptr(&gid) {
                Ok(ptr) => {
                    gpu::drop_buffer(self.gid.as_mut().unwrap());
                    self.ptr = ptr;
                    self.size = new_size;
                    self.gid = Some(gid);
                    Ok(ptr)
                }
                Err(e) => Err(e),
            }
        }
    }

    fn size(&self) -> usize {
        self.size
    }
}

impl Drop for VertAlloc {
    fn drop(&mut self) {
        // NOTE: Currently, `VarBuf` has a `drop`
        // implementation that calls `shrink(0)`,
        // so this will always be skipped.
        if let Some(ref mut gid) = self.gid {
            gpu::drop_buffer(gid);
        }
    }
}

/// Vertex buffer.
pub(crate) type VertBuf = VarBuf<VertAlloc>;

/// Mesh.
#[derive(Debug)]
pub struct Mesh {
    vert_buf: Arc<RwLock<VertBuf>>,
    primitives: Vec<Primitive>,
}

impl Mesh {
    /// Returns a reference to the reference-counted,
    /// r/w-locked vertex buffer.
    pub(crate) fn vertex_buffer(&self) -> &Arc<RwLock<VertBuf>> {
        &self.vert_buf
    }

    /// Returns a reference to the mesh's [`Primitive`]s.
    pub fn primitives(&self) -> &[Primitive] {
        &self.primitives
    }
}

/// Primitive.
#[derive(Debug)]
pub struct Primitive {
    // TODO: Displacement.
    semantics: [Option<(DataType, VarEntry)>; SEMANTIC_N],
    indices: Option<(DataType, VarEntry)>,
    material: Arc<Material>,
    topology: Topology,
}

impl Primitive {
    /// Returns a ([`DataType`], &[`VarEntry`]) pair representing
    /// a given semantic in memory, or [`None`] if such semantic
    /// is not present in this primitive.
    pub(crate) fn semantic_data(&self, sem: Semantic) -> Option<(DataType, &VarEntry)> {
        if let Some((d, ref v)) = self.semantics[sem as usize] {
            Some((d, v))
        } else {
            None
        }
    }

    /// Returns a ([`DataType`], &[`VarEntry`]) pair representing
    /// the indices in memory, or [`None`] if this primitive
    /// does not use an index buffer.
    pub(crate) fn index_data(&self) -> Option<(DataType, &VarEntry)> {
        if let Some((d, ref v)) = self.indices {
            Some((d, v))
        } else {
            None
        }
    }

    /// Returns the [`DataType`] used to store a given semantic,
    /// or [`None`] if such semantic is not present in this
    /// primitive.
    pub fn semantic_data_type(&self, sem: Semantic) -> Option<DataType> {
        if let Some((d, _)) = self.semantics[sem as usize] {
            Some(d)
        } else {
            None
        }
    }

    /// Returns the [`DataType`] used to store vertex indices,
    /// or [`None`] if this primitive does not use an
    /// index buffer.
    pub fn index_data_type(&self) -> Option<DataType> {
        if let Some((d, _)) = self.indices {
            Some(d)
        } else {
            None
        }
    }

    /// Returns a reference to the reference-counted [`Material`].
    pub fn material(&self) -> &Arc<Material> {
        &self.material
    }

    /// Returns the [`Topology`] used to draw this primitive.
    pub fn topology(&self) -> Topology {
        self.topology
    }
}

/// Semantics.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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

pub(crate) const SEMANTIC_N: usize = Semantic::Weights0 as usize + 1;

/// Data types.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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

/// Primitive topology values.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Topology {
    Point,
    Line,
    LineStrip,
    Triangle,
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

    pub fn set_weights(&mut self, weights: &[f64]) -> &mut Self {
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

    pub fn set_displacement_semantic(
        &mut self,
        slot: usize,
        semantic: Semantic,
        data_type: DataType,
        offset: usize,
        stride: usize,
    ) -> &mut Self {
        todo!();
    }

    pub fn read_displacement_semantic<T: Read>(
        &mut self,
        slot: usize,
        semantic: Semantic,
        mut reader: T,
    ) -> io::Result<&mut Self> {
        todo!();
    }

    pub fn set_material(&mut self, material: &Arc<Material>) -> &mut Self {
        todo!();
    }

    pub fn push_primitive(&mut self, topology: Topology) -> io::Result<&mut Self> {
        todo!();
    }

    pub fn create(&mut self) -> io::Result<Mesh> {
        todo!();
    }
}
