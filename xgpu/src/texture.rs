//! GPU texture.

use std::ops::{BitOr, RangeBounds};

use crate::Result;

pub struct Texture {
    // TODO
}

impl Texture {
    pub fn width(&self) -> u32 {
        panic!("not yet implemented");
    }

    pub fn height(&self) -> u32 {
        panic!("not yet implemented");
    }

    pub fn depth_or_layers(&self) -> u32 {
        panic!("not yet implemented");
    }

    pub fn level_count(&self) -> u32 {
        panic!("not yet implemented");
    }

    pub fn sample_count(&self) -> u32 {
        panic!("not yet implemented");
    }

    pub fn dimension(&self) -> TextureDimension {
        panic!("not yet implemented");
    }

    pub fn format(&self) -> TextureFormat {
        panic!("not yet implemented");
    }

    pub fn usage(&self) -> TextureUsageFlags {
        panic!("not yet implemented");
    }

    pub fn create_view<T, U>(&mut self, _desc: &TextureViewDescriptor<T, U>) -> Result<TextureView>
    where
        T: RangeBounds<u32>,
        U: RangeBounds<u32>,
    {
        panic!("not yet implemented");
    }
}

pub struct TextureDescriptor<'a> {
    pub size: Extent3d,
    pub level_count: u32,
    pub sample_count: u32,
    pub dimension: TextureDimension,
    pub format: TextureFormat,
    pub usage: TextureUsageFlags,
    pub view_formats: &'a [TextureFormat],
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Origin2d {
    pub x: u32,
    pub y: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Origin3d {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Extent3d {
    pub width: u32,
    pub height: u32,
    pub depth_or_layers: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TextureDimension {
    One,
    Two,
    Three,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TextureFormat {
    R8Unorm,
    R8Snorm,
    R8Uint,
    R8Sint,

    R16Uint,
    R16Sint,
    R16Float,
    Rg8Unorm,
    Rg8Snorm,
    Rg8Uint,
    Rg8Sint,

    R32Uint,
    R32Sint,
    R32Float,
    Rg16Uint,
    Rg16Sint,
    Rg16Float,
    Rgba8Unorm,
    Rgba8UnormSrgb,
    Rgba8Snorm,
    Rgba8Uint,
    Rgba8Sint,
    Bgra8Unorm,
    Bgra8UnormSrgb,
    Rgb9e5Ufloat,
    Rgb10a2Unorm,
    Rg11b10Ufloat,

    Rg32Uint,
    Rg32Sint,
    Rg32Float,
    Rgba16Uint,
    Rgba16Sint,
    Rgba16Float,

    Rgba32Uint,
    Rgba32Sint,
    Rgba32Float,

    Stencil8,
    Depth16Unorm,
    Depth24Plus,
    Depth24PlusStencil8,
    Depth32Float,

    // `Feature::Depth32FloatStencil8`
    Depth32FloatStencil8,

    // `Feature::TextureCompressionBc`
    Bc1RgbaUnorm,
    Bc1RgbaUnormSrgb,
    Bc2RgbaUnorm,
    Bc2RgbaUnormSrgb,
    Bc3RgbaUnorm,
    Bc3RgbaUnormSrgb,
    Bc4RUnorm,
    Bc4RSnorm,
    Bc5RgUnorm,
    Bc5RgSnorm,
    Bc6hRgbUfloat,
    Bc6hRgbFloat,
    Bc7RgbaUnorm,
    Bc7RgbaUnormSrgb,

    // `Feature::TextureCompressionEtc2`
    Etc2Rgb8Unorm,
    Etc2Rgb8UnormSrgb,
    Etc2Rgb8a1Unorm,
    Etc2Rgb8a1UnormSrgb,
    Etc2Rgba8Unorm,
    Etc2Rgba8UnormSrgb,
    EacR11Unorm,
    EacR11Snorm,
    EacRg11Unorm,
    EacRg11Snorm,

    // `Feature::TextureCompressionAstc`
    Astc4x4Unorm,
    Astc4x4UnormSrgb,
    Astc5x4Unorm,
    Astc5x4UnormSrgb,
    Astc5x5Unorm,
    Astc5x5UnormSrgb,
    Astc6x5Unorm,
    Astc6x5UnormSrgb,
    Astc6x6Unorm,
    Astc6x6UnormSrgb,
    Astc8x5Unorm,
    Astc8x5UnormSrgb,
    Astc8x6Unorm,
    Astc8x6UnormSrgb,
    Astc8x8Unorm,
    Astc8x8UnormSrgb,
    Astc10x5Unorm,
    Astc10x5UnormSrgb,
    Astc10x6Unorm,
    Astc10x6UnormSrgb,
    Astc10x8Unorm,
    Astc10x8UnormSrgb,
    Astc10x10Unorm,
    Astc10x10UnormSrgb,
    Astc12x10Unorm,
    Astc12x10UnormSrgb,
    Astc12x12Unorm,
    Astc12x12UnormSrgb,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum TextureUsage {
    CopySrc = 0x1,
    CopyDst = 0x2,
    TextureBinding = 0x4,
    StorageBinding = 0x8,
    RenderAttachment = 0x10,
}

impl BitOr for TextureUsage {
    type Output = TextureUsageFlags;

    fn bitor(self, rhs: Self) -> Self::Output {
        TextureUsageFlags(self as u16 | rhs as u16)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TextureUsageFlags(u16);

impl TextureUsageFlags {
    pub fn is_set(self, usage: TextureUsage) -> bool {
        self.0 & usage as u16 != 0
    }
}

impl BitOr<TextureUsage> for TextureUsageFlags {
    type Output = Self;

    fn bitor(self, rhs: TextureUsage) -> Self::Output {
        Self(self.0 | rhs as u16)
    }
}

impl From<TextureUsage> for TextureUsageFlags {
    fn from(value: TextureUsage) -> Self {
        Self(value as u16)
    }
}

pub struct TextureView {
    // TODO
}

pub struct TextureViewDescriptor<T, U>
where
    T: RangeBounds<u32>,
    U: RangeBounds<u32>,
{
    pub format: TextureFormat,
    pub dimension: TextureViewDimension,
    pub aspect: TextureAspect,
    pub level_range: T,
    pub layer_range: U,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TextureViewDimension {
    One,
    Two,
    TwoArray,
    Cube,
    CubeArray,
    Three,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TextureAspect {
    All,
    Stencil,
    Depth,
}
