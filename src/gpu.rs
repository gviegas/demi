// Copyright 2022 Gustavo C. Viegas. All rights reserved.

//! Interface to the graphics back-end.

use std::fmt;
use std::io;
use std::ptr::NonNull;

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
pub struct TexOptions {
    pub format: Format,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub levels: u32,
    pub samples: u32,
}

/// Graphics back-end interface.
// TODO
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
}
