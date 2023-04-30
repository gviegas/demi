// Copyright 2022 Gustavo C. Viegas. All rights reserved.

//! Punctual lights.

use crate::gpu::layout::LightU;
use crate::linear::Vec3;

/// Punctual light source.
#[derive(Debug)]
pub struct Light {
    light_type: LightType,
    unif: LightU,
    // TODO: Shadows.
}

/// Types of punctual lights.
#[derive(Copy, Clone, PartialEq, Debug)]
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
    pub fn new(mut light_type: LightType, intensity: f32, color: Vec3<f32>) -> Self {
        let (light, range, scale, offset) = match light_type {
            LightType::Directional => (LightU::DIRECTIONAL, 0.0, 0.0, 0.0),

            LightType::Point { mut range } => {
                range = range.clamp(1.0e-6, f32::MAX);
                light_type = LightType::Point { range };
                (LightU::POINT, range, 0.0, 0.0)
            }

            LightType::Spot {
                mut range,
                mut inner_angle,
                mut outer_angle,
            } => {
                range = range.clamp(1.0e-6, f32::MAX);
                outer_angle = outer_angle.clamp(1.0e-6, std::f32::consts::FRAC_PI_2);
                inner_angle = inner_angle.clamp(0.0, outer_angle - 1.0e-6);
                light_type = LightType::Spot {
                    range,
                    inner_angle,
                    outer_angle,
                };
                let inner_cos = inner_angle.cos();
                let outer_cos = outer_angle.cos();
                let cos_diff = inner_cos - outer_cos;
                let scale = if cos_diff < 1.0e-6 {
                    1.0e6
                } else {
                    1.0 / cos_diff
                };
                (LightU::SPOT, range, scale, scale * -outer_cos)
            }
        };
        // TODO: Consider clamping `intensity` and `color`.
        Self {
            light_type,
            unif: LightU {
                is_set: 1,
                light_type: light,
                intensity,
                range,
                color: color.into(),
                angular_scale: scale,
                position: [0.0; 3],
                angular_offset: offset,
                direction: [0.0, 0.0, -1.0],
                _pad: 0.0,
            },
        }
    }

    /// Creates a new punctual light whose color is pure white.
    pub fn new_white(light_type: LightType, intensity: f32) -> Self {
        Self::new(light_type, intensity, Vec3::from(1.0))
    }

    /// Returns the `LightType`.
    pub fn light_type(&self) -> LightType {
        self.light_type
    }

    /// Returns the intensity.
    pub fn intensity(&self) -> f32 {
        self.unif.intensity
    }

    /// Returns the color.
    pub fn color(&self) -> Vec3<f32> {
        Vec3::from(self.unif.color)
    }
}
