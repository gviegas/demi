// Copyright 2022 Gustavo C. Viegas. All rights reserved.

//! Interface to the graphics back-end.

use std::fmt;
use std::io;
use std::ptr::NonNull;

use crate::sampler::{Compare, Filter, Wrap};
use crate::texture::Format;

#[cfg(test)]
mod tests;

mod vk;

static mut IMPL: Option<Box<dyn Gpu>> = None;

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
}

/// GPU texture.
#[derive(Debug)]
pub struct TexId(Id);

/// Options for texture creation.
#[derive(Copy, Clone, Debug)]
pub struct TexOptions {
    pub format: Format,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub levels: u32,
    pub samples: u32,
}

/// GPU sampler.
#[derive(Debug)]
pub struct SplrId(Id);

/// Options for sampler creation.
#[derive(Copy, Clone, Debug)]
pub struct SplrOptions {
    pub u_wrap: Wrap,
    pub v_wrap: Wrap,
    pub w_wrap: Wrap,
    pub mag_filter: Filter,
    pub min_filter: (Filter, Option<Filter>),
    pub compare: Option<Compare>,
}

/// Graphics back-end interface.
///
/// NOTE: Keeping this trait private allow us to change it
/// without breaking non-`gpu` code.
///
// TODO...
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

    /// Creates a texture sampler.
    fn create_sampler(&self, options: &SplrOptions) -> io::Result<SplrId>;
}

/// Gets a reference to the `Gpu` implementation.
///
/// NOTE: The `Gpu` returned by this function is only guaranteed
/// to be valid if retrieved after a call to [`init`] and before
/// a call to [`shutdown`]. Attempts to use the `Gpu` outside of
/// this scope will lead to undefined behavior.
fn get<'a>() -> &'a dyn Gpu {
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
