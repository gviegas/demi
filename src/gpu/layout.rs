// Copyright 2023 Gustavo C. Viegas. All rights reserved.

//! Data as presented to the GPU.
//!
//! NOTE: These layouts are ongoing work and may change at any time.

/// Frame-global uniforms.
///
/// These values may differ between frames.
#[repr(C, align(16))]
pub struct FrameU {
    pub view_proj: [[f32; 4]; 4],
    pub view: [[f32; 4]; 4],
    pub proj: [[f32; 4]; 4],
    pub time: f32,
    pub rand: f32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub near: f32,
    pub far: f32,
    // TODO
    pub _pad: [f32; 8],
}

/// Light source uniforms.
///
/// The actual uniform is an array of [`LightU`].
///
/// These values may differ between frames.
#[repr(C, align(16))]
pub struct LightU {
    pub is_set: u32,
    pub light_type: u32,
    pub intensity: f32,
    pub range: f32,
    pub color: [f32; 3],
    pub angular_scale: f32,
    pub position: [f32; 3],
    pub angular_offset: f32,
    pub direction: [f32; 3],
    pub _pad: f32,
}

/// Drawable uniforms.
///
/// These values may differ between draw calls.
#[repr(C, align(16))]
pub struct DrawableU {
    pub world: [[f32; 4]; 4],
    pub normal: [[f32; 4]; 4],
    pub id: u32,
    pub flags: u32,
    // TODO
    pub _pad: [f32; 30],
}

/// Material uniforms.
///
/// These values may differ between draw calls.
#[repr(C, align(16))]
pub struct MaterialU {
    pub color_factor: [f32; 4],
    pub alpha_cutoff: f32,
    pub double_sided: u32,
    pub normal_factor: f32,
    pub occlusion_factor: f32,
    pub emissive_factor: [f32; 3],
    pub metalness: f32,
    pub roughness: f32,
    pub flags: u32,
    // TODO
    pub _pad: [f32; 2],
}

/// Skin's joint uniforms.
///
/// The actual uniform is an array of [`JointU`].
///
/// These values may differ between draw calls.
#[repr(C, align(16))]
pub struct JointU {
    pub joint: [[f32; 4]; 4],
    pub normal: [[f32; 4]; 4],
}

#[cfg(test)]
mod tests {
    use std::mem;

    use super::*;

    #[test]
    fn size_and_alignment() {
        assert_eq!(mem::size_of::<FrameU>(), 256);
        assert_eq!(mem::align_of::<FrameU>(), 16);

        assert_eq!(mem::size_of::<LightU>(), 64);
        assert_eq!(mem::align_of::<LightU>(), 16);

        assert_eq!(mem::size_of::<DrawableU>(), 256);
        assert_eq!(mem::align_of::<DrawableU>(), 16);

        assert_eq!(mem::size_of::<MaterialU>(), 64);
        assert_eq!(mem::align_of::<MaterialU>(), 16);

        assert_eq!(mem::size_of::<JointU>(), 128);
        assert_eq!(mem::align_of::<JointU>(), 16);
    }
}
