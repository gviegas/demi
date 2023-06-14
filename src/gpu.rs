// Copyright 2022 Gustavo C. Viegas. All rights reserved.

//! Interface to the graphics back-end.

use std::fmt;
use std::io;
use std::mem;
use std::ptr::NonNull;

use crate::sampler::{Compare, Filter, Wrap};
use crate::texture::Format;

#[cfg(test)]
mod tests;

pub mod layout;

#[cfg(any(target_os = "linux", windows))]
mod vk;

#[cfg(all(not(target_os = "linux"), not(windows)))]
compile_error!("platform not supported");

#[cfg(any(target_os = "linux", windows))]
type GpuImpl = vk::Impl;

#[cfg(all(not(target_os = "linux"), not(windows)))]
type GpuImpl = dyn Gpu;

/// [`Gpu`] must be object safe so we can turn [`IMPL`] into a
/// trait object if necessary.
const __IMPL: Option<Box<dyn Gpu>> = None;

static mut IMPL: Option<Box<GpuImpl>> = None;

/// Initializes the underlying implementation.
///
/// Panics if all back-ends fail to initialize.
///
/// NOTE: One must ensure this function is called exactly once,
/// before any `gpu` functionality is used. It is not safe to
/// call it from multiple threads.
#[cfg(any(target_os = "linux", windows))]
pub fn init() {
    unsafe {
        IMPL = Some(Box::new(
            vk::Impl::new().expect("no graphics back-end that we can use"),
        ));
    }
}

/// Drops the underlying implementation.
///
/// NOTE: One must ensure this function is called exactly once,
/// after all uses of `gpu`. It is not safe to call it from
/// multiple threads.
pub fn shutdown() {
    unsafe {
        IMPL = None;
    }
}

/// Identifier for GPU resources.
#[derive(Debug)]
pub enum Id {
    Ptr(NonNull<()>),
    Num(u64),
    Invalid,
}

/// GPU texture.
#[derive(Debug)]
pub struct TexId(Id);

/// Options for texture creation.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct TexOptions {
    pub format: Format,
    pub width: u32,
    pub height: u32,
    pub depth_or_layers: u32,
    pub levels: u32,
    pub samples: u32,
}

/// GPU sampler.
#[derive(Debug)]
pub struct SplrId(Id);

/// Options for sampler creation.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct SplrOptions {
    pub u_wrap: Wrap,
    pub v_wrap: Wrap,
    pub w_wrap: Wrap,
    pub mag_filter: Filter,
    pub min_filter: (Filter, Option<Filter>),
    pub compare: Option<Compare>,
}

/// GPU buffer.
#[derive(Debug)]
pub struct BufId(Id);

/// Options for buffer creation.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BufOptions {
    pub size: u64,
    pub cpu_visible: bool,
}

/// Graphics back-end interface.
///
/// NOTE: Keeping this trait private allow us to change it
/// without breaking non-`gpu` code.
trait Gpu: fmt::Display + fmt::Debug {
    /// Creates a 2D texture.
    ///
    /// This texture must support sampling in shaders and
    /// copies from/to buffer memory.
    fn create_2d(&self, options: &TexOptions) -> io::Result<TexId>;

    /// Creates a 3D texture.
    ///
    /// This texture must support sampling in shaders and
    /// copies from/to buffer memory.
    fn create_3d(&self, options: &TexOptions) -> io::Result<TexId>;

    /// Creates a cube texture.
    ///
    /// This texture must support sampling in shaders and
    /// copies from/to buffer memory.
    fn create_cube(&self, options: &TexOptions) -> io::Result<TexId>;

    /// Creates a render target texture.
    ///
    /// This texture must support sampling in shaders and
    /// copies from/to buffer memory.
    /// It must also be valid for use as either a color or
    /// depth/stencil render target.
    fn create_rt(&self, options: &TexOptions) -> io::Result<TexId>;

    /// Notifies that `tex_id` will no longer be used.
    ///
    /// The implementation is free to discard or reuse its
    /// resources.
    fn drop_texture(&self, tex_id: TexId);

    /// Creates a texture sampler.
    ///
    /// This sampler must be valid for use with any `TexId`.
    fn create_sampler(&self, options: &SplrOptions) -> io::Result<SplrId>;

    /// Notifies that `splr_id` will no longer be used.
    ///
    /// The implementation is free to discard or reuse its
    /// resources.
    fn drop_sampler(&self, splr_id: SplrId);

    /// Creates a vertex buffer.
    ///
    /// This buffer must support storage of vertices and
    /// indices for primitive drawing.
    fn create_vb(&self, options: &BufOptions) -> io::Result<BufId>;

    /// Creates an uniform buffer.
    ///
    /// This buffer must support storage of constant data
    /// for shader read access.
    fn create_ub(&self, options: &BufOptions) -> io::Result<BufId>;

    /// Gets a pointer to buffer memory.
    ///
    /// This method will always fail if the buffer is not
    /// CPU-visible.
    // TODO: Replace with opaque r/w methods (will break mesh::VertBuf).
    fn buffer_ptr(&self, buf_id: &BufId) -> io::Result<NonNull<()>>;

    /// Notifies that `buf_id` will no longer be used.
    ///
    /// The implementation is free to discard or reuse its
    /// resources.
    fn drop_buffer(&self, buf_id: BufId);
}

/// Gets a reference to the `Gpu` implementation.
///
/// NOTE: The `Gpu` returned by this function is only guaranteed
/// to be valid if retrieved after a call to [`init`] and before
/// a call to [`shutdown`]. Attempts to use the `Gpu` outside of
/// this scope will lead to undefined behavior.
fn get<'a>() -> &'a GpuImpl {
    unsafe {
        debug_assert!(IMPL.is_some());
        &**IMPL.as_ref().unwrap_unchecked()
    }
}

/// Creates a 2D texture.
pub fn create_2d(options: &TexOptions) -> io::Result<TexId> {
    get().create_2d(options)
}

/// Creates a 3D texture.
pub fn create_3d(options: &TexOptions) -> io::Result<TexId> {
    get().create_3d(options)
}

/// Creates a cube texture.
pub fn create_cube(options: &TexOptions) -> io::Result<TexId> {
    get().create_cube(options)
}

/// Creates a render target texture.
pub fn create_rt(options: &TexOptions) -> io::Result<TexId> {
    get().create_rt(options)
}

/// Notifies that `tex_id` will no longer be used.
pub fn drop_texture(tex_id: &mut TexId) {
    let tex_id = mem::replace(tex_id, TexId(Id::Invalid));
    get().drop_texture(tex_id);
}

/// Creates a texture sampler.
pub fn create_sampler(options: &SplrOptions) -> io::Result<SplrId> {
    get().create_sampler(options)
}

/// Notifies that `splr_id` will no longer be used.
pub fn drop_sampler(splr_id: &mut SplrId) {
    let splr_id = mem::replace(splr_id, SplrId(Id::Invalid));
    get().drop_sampler(splr_id);
}

/// Creates a vertex buffer.
pub fn create_vb(options: &BufOptions) -> io::Result<BufId> {
    get().create_vb(options)
}

/// Creates an uniform buffer.
pub fn create_ub(options: &BufOptions) -> io::Result<BufId> {
    get().create_ub(options)
}

/// Gets a pointer to buffer memory.
pub fn buffer_ptr(buf_id: &BufId) -> io::Result<NonNull<()>> {
    get().buffer_ptr(buf_id)
}

/// Notifies that `buf_id` will no longer be used.
pub fn drop_buffer(buf_id: &mut BufId) {
    let buf_id = mem::replace(buf_id, BufId(Id::Invalid));
    get().drop_buffer(buf_id);
}
