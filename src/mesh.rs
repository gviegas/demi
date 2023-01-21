// Copyright 2022 Gustavo C. Viegas. All rights reserved.

//! Geometry for drawing.

use std::alloc::Layout;
use std::io::{self, Read};
use std::mem::{self, MaybeUninit};
use std::ptr::NonNull;
use std::sync::{Arc, RwLock};

use crate::gpu::{self, BufId, BufOptions};
use crate::material::Material;
use crate::var_buf::{VarAlloc, VarBuf, VarEntry};

static mut VERT_BUF: Option<Arc<RwLock<VertBuf>>> = None;

/// Initializes the vertex buffer.
///
/// NOTE: One must ensure this function is called exactly once,
/// before any `mesh` functionality is used and after
/// initializing the `gpu`. It is not safe to call it from
/// multiple threads.
pub(crate) fn init() {
    unsafe {
        debug_assert!(VERT_BUF.is_none());
        // TODO: Consider making the initial allocation size
        // configurable.
        VERT_BUF = Some(Arc::new(RwLock::new(VertBuf::new(VertAlloc::new(0)))));
    }
}

/// Drops the vertex buffer.
///
/// NOTE: One must ensure this function is called exactly once,
/// after all uses of `mesh` and before finalizing the `gpu`.
/// It is not safe to call it from multiple threads.
pub(crate) fn shutdown() {
    unsafe {
        debug_assert!(Arc::get_mut(VERT_BUF.as_mut().unwrap()).is_some());
        VERT_BUF.take();
    }
}

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
    // NOTE: This alignment value should suffice for all
    // `gpu` back-ends (the widest `DataType` variants
    // currently defined have 32 bits per component).
    // It can be increased if necessary.
    const MIN_ALIGN: usize = 4;

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

/// Gets a reference-counted, r/w-locked `VertBuf`.
///
/// It is only safe to call this function after `init` completes and
/// before calling `shutdown` (notice that neither is thread-safe).
///
/// NOTE: This value (and its clones) must be dropped before
/// `shutdown` is called.
fn vertex_buffer() -> Arc<RwLock<VertBuf>> {
    unsafe { Arc::clone(VERT_BUF.as_ref().unwrap()) }
}

/// Mesh.
#[derive(Debug)]
pub struct Mesh(Vec<Primitive>);

impl Mesh {
    /// Returns a reference to the mesh's [`Primitive`]s.
    pub fn primitives(&self) -> &[Primitive] {
        &self.0
    }
}

/// Primitive.
#[derive(Debug)]
pub struct Primitive {
    // NOTE: In case we decide (or need) to use
    // multiple vertex buffers.
    vert_buf: Arc<RwLock<VertBuf>>,
    semantics: [Option<DataEntry>; SEMANTIC_N],
    indices: Option<DataEntry>,
    // Number of vertices to draw.
    // How to interpret them depends on whether
    // the primitive has `indices`.
    count: usize,
    material: Arc<Material>,
    // TODO: Do we really need `DataType` here?
    // Can we limit this to a subset of `Semantic`s?
    displacements: Vec<[Option<DataEntry>; SEMANTIC_N]>,
    weights: Vec<f32>,
    topology: Topology,
}

impl Primitive {
    /// Returns a reference to the reference-counted,
    /// r/w-locked vertex buffer.
    pub(crate) fn vertex_buffer(&self) -> &Arc<RwLock<VertBuf>> {
        &self.vert_buf
    }

    /// Returns a reference to [`DataEntry`] representing a given
    /// semantic in memory, or [`None`] if such semantic is not
    /// present in this primitive.
    pub(crate) fn semantic_data(&self, sem: Semantic) -> Option<&DataEntry> {
        if let Some(ref x) = self.semantics[sem as usize] {
            Some(x)
        } else {
            None
        }
    }

    /// Returns a reference to [`DataEntry`] representing the
    /// indices in memory, or [`None`] if this primitive
    /// does not use an index buffer.
    pub(crate) fn index_data(&self) -> Option<&DataEntry> {
        if let Some(ref x) = self.indices {
            Some(x)
        } else {
            None
        }
    }

    /// Returns a vector containing references to [`DataEntry`]
    /// that represent the displacements of `sem`, alongside
    /// the displacement slots which they refer.
    pub(crate) fn displacement_data(&self, sem: Semantic) -> Vec<(&DataEntry, usize)> {
        self.displacements
            .iter()
            .enumerate()
            .filter_map(|(i, x)| {
                if let Some(ref x) = x[sem as usize] {
                    Some((x, i))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Returns the [`DataType`] used to store a given semantic,
    /// or [`None`] if such semantic is not present in this
    /// primitive.
    pub fn semantic_data_type(&self, sem: Semantic) -> Option<DataType> {
        if let Some(DataEntry { data_type, .. }) = self.semantics[sem as usize] {
            Some(data_type)
        } else {
            None
        }
    }

    /// Returns the [`DataType`] used to store vertex indices,
    /// or [`None`] if this primitive does not use an
    /// index buffer.
    pub fn index_data_type(&self) -> Option<DataType> {
        if let Some(DataEntry { data_type, .. }) = self.indices {
            Some(data_type)
        } else {
            None
        }
    }

    /// Returns the number of vertices that are draw when drawing
    /// this primitive.
    ///
    /// NOTE: This value is to be interpreted as the number of indices
    /// to fetch from the index buffer, if one is present.
    pub fn vertex_count(&self) -> usize {
        self.count
    }

    /// Returns a reference to the reference-counted [`Material`].
    pub fn material(&self) -> &Arc<Material> {
        &self.material
    }

    /// Returns the number of displacement slots in this primitive.
    pub fn displacement_slots(&self) -> usize {
        self.displacements.len()
    }

    /// Returns a vector containing the [`DataType`]s used to
    /// store displacements for `sem`, alongside the displacement
    /// slots which they refer.
    pub fn displacement_data_type(&self, sem: Semantic) -> Vec<(DataType, usize)> {
        self.displacements
            .iter()
            .enumerate()
            .filter_map(|(i, x)| {
                if let Some(DataEntry { data_type, .. }) = x[sem as usize] {
                    Some((data_type, i))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Returns a reference to a slice containing the default weight
    /// of each displacement slot.
    pub fn weights(&self) -> &[f32] {
        &self.weights
    }

    /// Returns the [`Topology`] used to draw this primitive.
    pub fn topology(&self) -> Topology {
        self.topology
    }
}

/// Description of mesh data in memory.
///
/// Data is stored tightly packed as defined by `DataType::layout`.
///
/// Each semantic of a primitive is guaranteed to be laid out
/// contiguously in memory. The same applies for index data.
#[derive(Debug)]
pub(crate) struct DataEntry {
    data_type: DataType,
    entry: VarEntry,
}

impl DataEntry {
    /// Returns the [`DataType`].
    pub fn data_type(&self) -> DataType {
        self.data_type
    }

    /// Returns a reference to the [`VarEntry`].
    pub fn entry(&self) -> &VarEntry {
        &self.entry
    }
}

/// Data types.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum DataType {
    F32,
    F32x2,
    F32x3,
    F32x4,
    I32,
    I32x2,
    I32x3,
    I32x4,
    U32,
    U32x2,
    U32x3,
    U32x4,
    I16,
    I16x2,
    I16x3,
    I16x4,
    U16,
    U16x2,
    U16x3,
    U16x4,
    I8,
    I8x2,
    I8x3,
    I8x4,
    U8,
    U8x2,
    U8x3,
    U8x4,
}

impl DataType {
    /// Returns the `[Layout]` of the [`DataType`].
    pub const fn layout(&self) -> Layout {
        match *self {
            DataType::F32 | DataType::I32 | DataType::U32 => Layout::new::<f32>(),
            DataType::F32x2 | DataType::I32x2 | DataType::U32x2 => Layout::new::<[f32; 2]>(),
            DataType::F32x3 | DataType::I32x3 | DataType::U32x3 => Layout::new::<[f32; 3]>(),
            DataType::F32x4 | DataType::I32x4 | DataType::U32x4 => Layout::new::<[f32; 4]>(),
            DataType::I16 | DataType::U16 => Layout::new::<i16>(),
            DataType::I16x2 | DataType::U16x2 => Layout::new::<[i16; 2]>(),
            DataType::I16x3 | DataType::U16x3 => Layout::new::<[i16; 3]>(),
            DataType::I16x4 | DataType::U16x4 => Layout::new::<[i16; 4]>(),
            DataType::I8 | DataType::U8 => Layout::new::<i8>(),
            DataType::I8x2 | DataType::U8x2 => Layout::new::<[i8; 2]>(),
            DataType::I8x3 | DataType::U8x3 => Layout::new::<[i8; 3]>(),
            DataType::I8x4 | DataType::U8x4 => Layout::new::<[i8; 4]>(),
        }
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

/// Constructs an array of `SEMANTIC_N` `Option<DataEntry>`s
/// where each element is `None`.
fn none_semantics() -> [Option<DataEntry>; SEMANTIC_N] {
    unsafe {
        let mut sems: [MaybeUninit<Option<DataEntry>>; SEMANTIC_N] =
            MaybeUninit::uninit().assume_init();
        for i in &mut sems {
            i.write(None);
        }
        mem::transmute::<_, [Option<DataEntry>; SEMANTIC_N]>(sems)
    }
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
    vert_buf: Arc<RwLock<VertBuf>>,
    // Data of the primitive being built,
    // which will be consumed by the next
    // `push_primitive` call.
    semantics: [Option<DataEntry>; SEMANTIC_N],
    indices: Option<DataEntry>,
    vert_count: usize,
    idx_count: usize,
    material: Option<Arc<Material>>,
    displacements: Vec<[Option<DataEntry>; SEMANTIC_N]>,
    weights: Vec<f32>,
    // Pushed primitives.
    // Each new element pushed here consumes
    // the per-primitive fields above.
    primitives: Vec<Primitive>,
    mask: u32,
}

impl Builder {
    const FROZEN_VERT_COUNT: u32 = 1 << 0;
    const POSITION: u32 = 1 << 1;

    /// Creates a new mesh builder.
    pub fn new() -> Self {
        debug_assert!(unsafe { VERT_BUF.is_some() });
        Self {
            vert_buf: vertex_buffer(),
            semantics: none_semantics(),
            indices: None,
            vert_count: 0,
            idx_count: 0,
            material: None,
            displacements: vec![],
            weights: vec![],
            // The (expected) common case.
            primitives: Vec::with_capacity(1),
            mask: 0,
        }
    }

    /// Sets the vertex count.
    ///
    /// This value indicates the number of data elements to fetch
    /// when reading semantic input.
    /// All semantics (including displacements) must have the same
    /// vertex count.
    ///
    /// Panics if `count` is zero or if setting it would invalidate
    /// the ongoing primitive.
    pub fn set_vertex_count(&mut self, count: usize) -> &mut Self {
        assert_ne!(count, 0);
        if count != self.vert_count {
            assert_eq!(self.mask & Self::FROZEN_VERT_COUNT, 0);
            self.vert_count = count;
        }
        self
    }

    /// Sets semantic data.
    ///
    /// This method sets the given semantic to contain `data_type`
    /// elements, each of which is fetched `stride` bytes apart from
    /// `reader`.
    /// The number of [`DataType`] elements to read is defined by
    /// `set_vertex_count`.
    pub fn set_semantic<T: Read>(
        &mut self,
        mut reader: T,
        semantic: Semantic,
        data_type: DataType,
        stride: usize,
    ) -> io::Result<&mut Self> {
        let layout = data_type.layout();
        debug_assert!(VertAlloc::MIN_ALIGN >= layout.align());
        if self.vert_count == 0 {
            return Err(io::Error::from(io::ErrorKind::Other));
        }
        // This should not happen in practice, but we guard
        // against it anyway. We will not try anything
        // fancy like reusing the entry though.
        if let Some(DataEntry { entry, .. }) = self.semantics[semantic as usize].take() {
            eprintln!(
                "[!] mesh::Builder: set_semantic called twice for {:?}",
                semantic
            );
            self.vert_buf.write().unwrap().dealloc(entry);
            if semantic == Semantic::Position {
                self.mask &= !Self::POSITION;
            }
        }
        // No padding between `data_type` elements.
        let size = layout.size() * self.vert_count;
        let entry = self.vert_buf.write().unwrap().alloc(size)?;
        let mut buf = vec![0u8; size];
        if stride == 0 || stride == layout.size() {
            match reader.read_exact(&mut buf) {
                Ok(_) => (),
                Err(e) => {
                    self.vert_buf.write().unwrap().dealloc(entry);
                    return Err(e);
                }
            }
        } else {
            todo!();
        }
        self.vert_buf.write().unwrap().copy(&buf, &entry);
        self.semantics[semantic as usize] = Some(DataEntry { data_type, entry });
        // Do not allow the vertex count to change
        // for this primitive anymore.
        self.mask |= Self::FROZEN_VERT_COUNT;
        if semantic == Semantic::Position {
            // Position is mandatory.
            self.mask |= Self::POSITION;
        }
        Ok(self)
    }

    /// Sets vertex indices.
    ///
    /// This method sets the index buffer to contain `count`
    /// `data_type` elements fetched from `reader`.
    /// The data is assumed to be tightly packed.
    pub fn set_indexed<T: Read>(
        &mut self,
        mut reader: T,
        count: usize,
        data_type: DataType,
    ) -> io::Result<&mut Self> {
        debug_assert!(VertAlloc::MIN_ALIGN >= 4);
        if count == 0 {
            return Err(io::Error::from(io::ErrorKind::InvalidInput));
        }
        let (data_size, stride) = match data_type {
            DataType::U32 => (4, 4),
            DataType::U16 => (2, 2),
            // We will extend `DataType::U8` to 16-bit,
            // since it is not universally supported.
            DataType::U8 => (1, 2),
            _ => return Err(io::Error::from(io::ErrorKind::InvalidInput)),
        };
        if let Some(DataEntry { entry, .. }) = self.indices.take() {
            eprintln!("[!] mesh::Builder: set_indexed called twice");
            self.vert_buf.write().unwrap().dealloc(entry);
            self.idx_count = 0;
        }
        let size = stride * count;
        let entry = self.vert_buf.write().unwrap().alloc(size)?;
        let mut buf = vec![0u8; size];
        if data_size == stride {
            match reader.read_exact(&mut buf) {
                Ok(_) => (),
                Err(e) => {
                    self.vert_buf.write().unwrap().dealloc(entry);
                    return Err(e);
                }
            }
        } else {
            todo!();
        }
        self.vert_buf.write().unwrap().copy(&buf, &entry);
        self.indices = Some(DataEntry { data_type, entry });
        self.idx_count = count;
        Ok(self)
    }

    /// Sets the material.
    pub fn set_material(&mut self, material: &Arc<Material>) -> &mut Self {
        self.material = Some(Arc::clone(material));
        self
    }

    /// Sets displacement data.
    ///
    /// This method sets the given semantic of the given displacement
    /// slot to contain `data_type` elements, each of which is
    /// fetched `stride` bytes apart from `reader`.
    /// The number of [`DataType`] elements to read is defined by
    /// `set_vertex_count`.
    pub fn set_displacement_semantic<T: Read>(
        &mut self,
        mut reader: T,
        slot: usize,
        semantic: Semantic,
        data_type: DataType,
        stride: usize,
    ) -> io::Result<&mut Self> {
        let layout = data_type.layout();
        debug_assert!(VertAlloc::MIN_ALIGN >= layout.align());
        if self.vert_count == 0 {
            return Err(io::Error::from(io::ErrorKind::Other));
        }
        if slot >= self.displacements.len() {
            // NOTE: This generates empty slots in the range
            // `self.displacements.len()..slot`.
            self.displacements.resize_with(slot + 1, none_semantics);
        } else if let Some(DataEntry { entry, .. }) =
            self.displacements[slot][semantic as usize].take()
        {
            eprintln!(
                "[!] mesh::Builder: set_displacement_semantic called twice for [{}] {:?}",
                slot, semantic
            );
            self.vert_buf.write().unwrap().dealloc(entry);
        }
        let size = layout.size() * self.vert_count;
        let entry = self.vert_buf.write().unwrap().alloc(size)?;
        let mut buf = vec![0u8; size];
        if stride == 0 || stride == layout.size() {
            match reader.read_exact(&mut buf) {
                Ok(_) => (),
                Err(e) => {
                    self.vert_buf.write().unwrap().dealloc(entry);
                    return Err(e);
                }
            }
        } else {
            todo!();
        }
        self.vert_buf.write().unwrap().copy(&buf, &entry);
        self.displacements[slot][semantic as usize] = Some(DataEntry { data_type, entry });
        // Currently, we do not support mismatch between
        // displacement and vertex counts.
        self.mask |= Self::FROZEN_VERT_COUNT;
        Ok(self)
    }

    /// Sets the default displacement weights.
    ///
    /// NOTE: The length of this vector must match the number of
    /// displacement slots used.
    pub fn set_weights(&mut self, weights: Vec<f32>) -> &mut Self {
        self.weights = weights;
        self
    }

    /// Consumes the current state to create a [`Primitive`].
    ///
    /// If this method fails, the state is left untouched.
    /// One may call `clear_primitive` to start over.
    pub fn push_primitive(&mut self, topology: Topology) -> io::Result<&mut Self> {
        // Check correctness before consuming any state.
        let err = io::Error::from(io::ErrorKind::InvalidInput);
        if self.mask & Self::POSITION == 0 {
            eprintln!("[!] mesh::Builder: primitives must have position semantic");
            return Err(err);
        }
        if self.displacements.len() != self.weights.len() {
            eprintln!(
                "[!] mesh::Builder: primitives must have a weight for each displacement slot"
            );
            return Err(err);
        }
        let count = if self.idx_count > 0 {
            self.idx_count
        } else {
            self.vert_count
        };
        let print_top_err = || {
            eprintln!(
                "[!] mesh::Builder: `{}` is not a valid number of vertices for {:?}",
                count, topology
            )
        };
        match topology {
            Topology::Point => (),
            Topology::Line => {
                if count & 1 != 0 {
                    print_top_err();
                    return Err(err);
                }
            }
            Topology::LineStrip => {
                if count < 2 {
                    print_top_err();
                    return Err(err);
                }
            }
            Topology::Triangle => {
                if count % 3 != 0 {
                    print_top_err();
                    return Err(err);
                }
            }
            Topology::TriangleStrip | Topology::TriangleFan => {
                if count < 3 {
                    print_top_err();
                    return Err(err);
                }
            }
        }
        // TODO: More checks.

        // Now we can consume the state.
        let mut semantics = none_semantics();
        mem::swap(&mut semantics, &mut self.semantics);
        let indices = mem::take(&mut self.indices);
        self.vert_count = 0;
        self.idx_count = 0;
        // TODO: Default material.
        let material = self.material.take().expect("no default material yet");
        let displacements = mem::take(&mut self.displacements);
        let weights = mem::take(&mut self.weights);
        self.primitives.push(Primitive {
            vert_buf: Arc::clone(&self.vert_buf),
            semantics,
            indices,
            count,
            material,
            displacements,
            weights,
            topology,
        });
        self.mask = 0;
        Ok(self)
    }

    /// Clears the current primitive state.
    pub fn clear_primitive(&mut self) -> &mut Self {
        // TODO: It may be better locking at `dealloc`
        // call sites.
        let mut vb = self.vert_buf.write().unwrap();
        for i in &mut self.semantics {
            if let Some(DataEntry { entry, .. }) = i.take() {
                vb.dealloc(entry);
            }
        }
        if let Some(DataEntry { entry, .. }) = self.indices.take() {
            vb.dealloc(entry);
        }
        self.vert_count = 0;
        self.idx_count = 0;
        self.material = None;
        while let Some(x) = self.displacements.pop() {
            for i in x {
                if let Some(DataEntry { entry, .. }) = i {
                    vb.dealloc(entry);
                }
            }
        }
        self.weights = vec![];
        self.mask = 0;
        drop(vb);
        self
    }

    /// Creates the mesh.
    ///
    /// This method consumes every [`Primitive`] that has been
    /// pushed up to this point.
    /// The current primitive state is unaffected.
    ///
    /// Fails if no primitive has been pushed yet.
    pub fn create(&mut self) -> io::Result<Mesh> {
        if self.primitives.len() > 0 {
            Ok(Mesh(mem::take(&mut self.primitives)))
        } else {
            Err(io::Error::from(io::ErrorKind::InvalidInput))
        }
    }
}

impl Drop for Builder {
    fn drop(&mut self) {
        self.clear_primitive();
    }
}
