// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use std::io;

/// Texture.
pub struct Texture {
    // TODO
}

/// Texture pixel formats.
pub enum Format {
    Xrgb8888,
    Argb8888,
    Bgra8888,
    Rgba8888,
    Rgba16161616,
    GenericLdr,
    GenericHdr,
    GenericDepth,
    GenericDepthStencil,
    CompressedLdr,
    CompressedHdr,
    // TODO
}

/// Texture builder.
pub struct Builder {
    // TODO
}

#[allow(unused_variables)] // TODO
impl Builder {
    pub fn new() -> Self {
        todo!();
    }

    pub fn set_format(&mut self, format: Format) -> &mut Self {
        todo!();
    }

    pub fn set_size(&mut self, width: u32, height: u32, depth: u32) -> &mut Self {
        todo!();
    }

    pub fn set_mipmap(&mut self, levels: u32) -> &mut Self {
        todo!();
    }

    pub fn set_multisample(&mut self, samples: u32) -> &mut Self {
        todo!()
    }

    pub fn create_2d(&mut self) -> io::Result<Texture> {
        todo!();
    }

    pub fn create_3d(&mut self) -> io::Result<Texture> {
        todo!();
    }

    pub fn create_cube(&mut self) -> io::Result<Texture> {
        todo!();
    }

    pub fn create_rt(&mut self) -> io::Result<Texture> {
        todo!();
    }
}
