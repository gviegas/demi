// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use std::io;

/// Material.
pub struct Material {
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

    pub fn set_base_color(&mut self, texture: Option<&TexRef>, factor: [f32; 4]) -> &mut Self {
        todo!();
    }

    pub fn set_metallic_roughness(
        &mut self,
        texture: Option<&TexRef>,
        metalness: f32,
        roughness: f32,
    ) -> &mut Self {
        todo!();
    }

    pub fn set_normal(&mut self, texture: Option<&TexRef>, scale: f32) -> &mut Self {
        todo!();
    }

    pub fn set_occlusion(&mut self, texture: Option<&TexRef>, strength: f32) -> &mut Self {
        todo!();
    }

    pub fn set_emissive(&mut self, texture: Option<&TexRef>, factor: [f32; 3]) -> &mut Self {
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

    pub fn create_unlit(&mut self) -> io::Result<Material> {
        todo!();
    }
}