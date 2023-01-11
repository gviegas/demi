// Copyright 2022 Gustavo C. Viegas. All rights reserved.

//! Material models.

use std::io;
use std::sync::Arc;

use crate::gpu::layout::MaterialU;
use crate::sampler::Sampler;
use crate::texture::Texture;

/// Material.
#[derive(Debug)]
pub struct Material {
    base_color_tex: Option<TexRef>,
    metal_rough_tex: Option<TexRef>,
    normal_tex: Option<TexRef>,
    occlusion_tex: Option<TexRef>,
    emissive_tex: Option<TexRef>,
    unif: MaterialU,
}

/// Reference to a texture and sampler.
#[derive(Clone, Debug)]
pub struct TexRef {
    texture: Arc<Texture>,
    layer: usize,
    sampler: Arc<Sampler>,
}

impl TexRef {
    /// Creates a new texture/sampler reference.
    pub fn new(texture: &Arc<Texture>, layer: usize, sampler: &Arc<Sampler>) -> Self {
        Self {
            texture: Arc::clone(texture),
            layer,
            sampler: Arc::clone(sampler),
        }
    }

    /// Returns a reference to the texture.
    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    /// Returns the layer index.
    pub fn layer(&self) -> usize {
        self.layer
    }

    /// Returns a reference to the sampler.
    pub fn sampler(&self) -> &Sampler {
        &self.sampler
    }
}

/// Alpha modes.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AlphaMode {
    Opaque,
    Blend,
    Mask { cutoff: f32 },
}

/// Material builder.
pub struct Builder<'a> {
    base_color: (Option<&'a TexRef>, [f32; 4]),
    metallic_roughness: (Option<&'a TexRef>, f32, f32),
    normal: (Option<&'a TexRef>, f32),
    occlusion: (Option<&'a TexRef>, f32),
    emissive: (Option<&'a TexRef>, [f32; 3]),
    alpha_mode: AlphaMode,
    double_sided: bool,
}

impl<'a> Builder<'a> {
    /// Creates a new material builder.
    pub fn new() -> Self {
        Self {
            base_color: (None, [1.0; 4]),
            metallic_roughness: (None, 1.0, 1.0),
            normal: (None, 1.0),
            occlusion: (None, 1.0),
            emissive: (None, [0.0; 3]),
            alpha_mode: AlphaMode::Opaque,
            double_sided: false,
        }
    }

    /// Sets the base color.
    ///
    /// These values need not be set. It defaults to a pure white, opaque color.
    pub fn set_base_color(&mut self, texture: Option<&'a TexRef>, factor: [f32; 4]) -> &mut Self {
        self.base_color = (texture, factor);
        self
    }

    /// Sets the metallic-roughness.
    ///
    /// These values need not be set. It defaults to fully metallic/rough.
    pub fn set_metallic_roughness(
        &mut self,
        texture: Option<&'a TexRef>,
        metalness: f32,
        roughness: f32,
    ) -> &mut Self {
        self.metallic_roughness = (texture, metalness, roughness);
        self
    }

    /// Sets the normal map.
    ///
    /// These values need not be set. Setting `texture` to [`None`]
    /// (the default) disables normal mapping.
    pub fn set_normal(&mut self, texture: Option<&'a TexRef>, scale: f32) -> &mut Self {
        self.normal = (texture, scale);
        self
    }

    /// Sets the occlusion map.
    ///
    /// These values need not be set. Setting `texture` to [`None`]
    /// (the default) disables occlusion mapping.
    pub fn set_occlusion(&mut self, texture: Option<&'a TexRef>, strength: f32) -> &mut Self {
        self.occlusion = (texture, strength);
        self
    }

    /// Sets the emissive map.
    ///
    /// These values need not be set. Setting `texture` to [`None`]
    /// (the default) disables emissive mapping.
    pub fn set_emissive(&mut self, texture: Option<&'a TexRef>, factor: [f32; 3]) -> &mut Self {
        self.emissive = (texture, factor);
        self
    }

    /// Sets the alpha mode.
    ///
    /// This value need not be set. It defaults to [`AlphaMode::Opaque`].
    pub fn set_alpha_mode(&mut self, mode: AlphaMode) -> &mut Self {
        self.alpha_mode = mode;
        self
    }

    /// Sets whether the material is double-sided.
    ///
    /// This value need not be set. It defaults to `false`.
    pub fn set_double_sided(&mut self, double_sided: bool) -> &mut Self {
        self.double_sided = double_sided;
        self
    }

    /// Creates a metallic-roughness material.
    pub fn create(&mut self) -> io::Result<Material> {
        // TODO: Consider letting the `Gpu` known about this.
        let (alpha_cutoff, flags) = match self.alpha_mode {
            AlphaMode::Opaque => (0.0, MaterialU::ALPHA_MODE_OPAQUE),
            AlphaMode::Blend => (0.0, MaterialU::ALPHA_MODE_BLEND),
            AlphaMode::Mask { cutoff } => (cutoff, MaterialU::ALPHA_MODE_MASK),
        };
        let flags = MaterialU::METALLIC_ROUGHNESS
            | if self.double_sided {
                MaterialU::DOUBLE_SIDED | flags
            } else {
                flags
            };
        Ok(Material {
            base_color_tex: self.base_color.0.cloned(),
            metal_rough_tex: self.metallic_roughness.0.cloned(),
            normal_tex: self.normal.0.cloned(),
            occlusion_tex: self.occlusion.0.cloned(),
            emissive_tex: self.emissive.0.cloned(),
            unif: MaterialU {
                base_color_factor: self.base_color.1,
                metalness: self.metallic_roughness.1,
                roughness: self.metallic_roughness.2,
                normal_scale: self.normal.1,
                occlusion_strength: self.occlusion.1,
                emissive_factor: self.emissive.1,
                alpha_cutoff,
                flags,
                _pad: Default::default(),
            },
        })
    }

    /// Creates an unlit material.
    ///
    /// The only properties that affect this material are
    /// the base color (texture and factor), the alpha mode
    /// and whether or not it is double-sided.
    pub fn create_unlit(&mut self) -> io::Result<Material> {
        // TODO: Consider letting the `Gpu` known about this.
        let (alpha_cutoff, flags) = match self.alpha_mode {
            AlphaMode::Opaque => (0.0, MaterialU::ALPHA_MODE_OPAQUE),
            AlphaMode::Blend => (0.0, MaterialU::ALPHA_MODE_BLEND),
            AlphaMode::Mask { cutoff } => (cutoff, MaterialU::ALPHA_MODE_MASK),
        };
        let flags = MaterialU::UNLIT
            | if self.double_sided {
                MaterialU::DOUBLE_SIDED | flags
            } else {
                flags
            };
        Ok(Material {
            base_color_tex: self.base_color.0.cloned(),
            metal_rough_tex: None,
            normal_tex: None,
            occlusion_tex: None,
            emissive_tex: None,
            unif: MaterialU {
                base_color_factor: self.base_color.1,
                metalness: 0.0,
                roughness: 0.0,
                normal_scale: 0.0,
                occlusion_strength: 0.0,
                emissive_factor: [0.0; 3],
                alpha_cutoff,
                flags,
                _pad: Default::default(),
            },
        })
    }
}
