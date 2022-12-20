// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use crate::linear::Vec3;

/// Punctual light source.
#[derive(Debug)]
pub struct Light {
    light_type: LightType,
    intensity: f32,
    color: Vec3<f32>,
    // TODO: Shadows.
}

/// Types of punctual lights.
#[derive(Copy, Clone, Debug)]
pub enum LightType {
    Directional,
    Point {
        range: f32,
    },
    Spot {
        range: f32,
        inner_angle: f32,
        outer_angle: f32,
    },
}

impl Light {
    /// Creates a new punctual light.
    pub fn new(light_type: LightType, intensity: f32, color: Vec3<f32>) -> Self {
        Self {
            light_type,
            intensity,
            color,
        }
    }

    /// Creates a new punctual light whose color is pure white.
    pub fn new_white(light_type: LightType, intensity: f32) -> Self {
        Self::new(light_type, intensity, Vec3::from(1.0))
    }

    // TODO: Setters.

    /// Returns the `LightType`.
    pub fn light_type(&self) -> LightType {
        self.light_type
    }

    /// Returns the intensity.
    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    /// Returns the color.
    pub fn color(&self) -> Vec3<f32> {
        self.color
    }
}
