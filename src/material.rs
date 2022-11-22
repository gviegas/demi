// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use std::io;

/// Material.
pub struct Material {
    // TODO
}

/// Material model.
pub enum Model {
    Pbrmr { metallic: f32, roughness: f32 },
    Unlit,
    // TODO
}

/// Reference to a texture and sampler.
pub struct TexRef {
    // TODO
}

/// Alpha modes.
pub enum AlphaMode {
    Opaque,
    Blend,
    Mask { cutoff: f32 },
}

/// Material builder.
pub struct Builder {
    // TODO
}

#[allow(unused_variables)] // TODO
impl Builder {
    pub fn new() -> Self {
        todo!();
    }

    pub fn set_model(&mut self, model: Model) -> &mut Self {
        todo!();
    }

    pub fn set_color_texture(&mut self, texture: Option<&TexRef>) -> &mut Self {
        todo!();
    }

    pub fn set_color_factor(&mut self, factor: [f32; 4]) -> &mut Self {
        todo!();
    }

    pub fn set_specular_texture(&mut self, texture: Option<&TexRef>) -> &mut Self {
        todo!();
    }

    pub fn set_normal_texture(&mut self, texture: Option<&TexRef>) -> &mut Self {
        todo!();
    }

    pub fn set_normal_scale(&mut self, scale: f32) -> &mut Self {
        todo!();
    }

    pub fn set_occlusion_texture(&mut self, texture: Option<&TexRef>) -> &mut Self {
        todo!();
    }

    pub fn set_occlusion_strength(&mut self, strength: f32) -> &mut Self {
        todo!();
    }

    pub fn set_emissive_texture(&mut self, texture: Option<&TexRef>) -> &mut Self {
        todo!();
    }

    pub fn set_emissive_factor(&mut self, factor: [f32; 3]) -> &mut Self {
        todo!();
    }

    pub fn set_alpha_mode(&mut self, mode: AlphaMode) -> &mut Self {
        todo!();
    }

    pub fn set_double_sided(&mut self, double_sided: bool) -> &mut Self {
        todo!();
    }

    pub fn create(&mut self) -> io::Result<Material> {
        todo!();
    }
}
