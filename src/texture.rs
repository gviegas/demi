//! Texture.

use std::io;

use crate::gpu::{self, TexId, TexOptions};

/// Texture.
#[derive(Debug)]
pub struct Texture {
    options: TexOptions,
    gid: TexId,
}

impl Texture {
    /// Returns the pixel format.
    pub fn format(&self) -> Format {
        self.options.format
    }

    /// Returns the width in pixels.
    pub fn width(&self) -> u32 {
        self.options.width
    }

    /// Returns the height in pixels.
    pub fn height(&self) -> u32 {
        self.options.height
    }

    /// Returns either the number of array layers (non-3D textures),
    /// or the depth in pixels.
    pub fn depth_or_layers(&self) -> u32 {
        self.options.depth_or_layers
    }

    /// Returns the number of mip levels.
    pub fn levels(&self) -> u32 {
        self.options.levels
    }

    /// Returns the sample count.
    pub fn samples(&self) -> u32 {
        self.options.samples
    }

    /// Returns a reference to the [`TexId`].
    pub(crate) fn tex_id(&self) -> &TexId {
        &self.gid
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        gpu::drop_texture(&mut self.gid);
    }
}

/// Texture pixel formats.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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
    options: TexOptions,
    mask: u32,
}

impl Builder {
    const FORMAT: u32 = 1 << 0;
    const SIZE: u32 = 1 << 1;
    const MASK: u32 = Self::FORMAT | Self::SIZE;

    /// Creates a new texture builder.
    pub fn new() -> Self {
        Self {
            options: TexOptions {
                format: Format::Rgba8888,
                width: 0,
                height: 0,
                depth_or_layers: 0,
                levels: 1,
                samples: 1,
            },
            mask: 0,
        }
    }

    /// Sets the pixel format.
    pub fn set_format(&mut self, format: Format) -> &mut Self {
        self.options.format = format;
        self.mask |= Self::FORMAT;
        self
    }

    /// Sets the dimensions of the texture.
    ///
    /// For non-3D textures, the `depth_or_layers` value is
    /// interpreted as the number of array layers instead of
    /// depth (i.e., it defines an arrayed texture).
    ///
    /// `width`, `height` and `depth_or_layers` must be
    /// greater than zero.
    pub fn set_size(&mut self, width: u32, height: u32, depth_or_layers: u32) -> &mut Self {
        assert!(width > 0);
        assert!(height > 0);
        assert!(depth_or_layers > 0);
        self.options.width = width;
        self.options.height = height;
        self.options.depth_or_layers = depth_or_layers;
        self.mask |= Self::SIZE;
        self
    }

    /// Sets the number of mip levels in the texture.
    ///
    /// `levels` must be greater than zero.
    ///
    /// This value need not be set. It defaults to one.
    pub fn set_mipmap(&mut self, levels: u32) -> &mut Self {
        assert!(levels > 0);
        self.options.levels = levels;
        self
    }

    /// Sets the number of samples in the texture.
    ///
    /// `samples` must be greater than zero and a power of two.
    ///
    /// This value need not be set. It defaults to one.
    pub fn set_multisample(&mut self, samples: u32) -> &mut Self {
        assert!(samples > 0);
        assert!(samples & (samples - 1) == 0);
        self.options.samples = samples;
        self
    }

    /// Creates a 2D texture.
    pub fn create_2d(&mut self) -> io::Result<Texture> {
        assert_eq!(self.mask & Self::MASK, Self::MASK);
        Ok(Texture {
            options: self.options,
            gid: gpu::create_2d(&self.options)?,
        })
    }

    /// Creates a 3D texture.
    pub fn create_3d(&mut self) -> io::Result<Texture> {
        assert_eq!(self.mask & Self::MASK, Self::MASK);
        Ok(Texture {
            options: self.options,
            gid: gpu::create_3d(&self.options)?,
        })
    }

    /// Creates a cube texture.
    pub fn create_cube(&mut self) -> io::Result<Texture> {
        assert_eq!(self.mask & Self::MASK, Self::MASK);
        Ok(Texture {
            options: self.options,
            gid: gpu::create_cube(&self.options)?,
        })
    }

    /// Creates a render target texture.
    pub fn create_rt(&mut self) -> io::Result<Texture> {
        assert_eq!(self.mask & Self::MASK, Self::MASK);
        Ok(Texture {
            options: self.options,
            gid: gpu::create_rt(&self.options)?,
        })
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}
