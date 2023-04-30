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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directional() {
        const WHITE: [f32; 3] = [1.0; 3];

        for i in [
            Light::new(LightType::Directional, 1000.0, WHITE.into()),
            Light::new_white(LightType::Directional, 1000.0),
        ] {
            assert_eq!(i.light_type, LightType::Directional);
            assert_eq!(i.unif.is_set, 1);
            assert_eq!(i.unif.light_type, LightU::DIRECTIONAL);
            assert_eq!(i.unif.intensity, 1000.0);
            assert_eq!(i.unif.color, WHITE);
        }
    }

    #[test]
    fn point() {
        const YELLOW: [f32; 3] = [1.0, 1.0, 0.0];

        let point = LightType::Point { range: 10.0 };

        let l = Light::new(point, 500.0, YELLOW.into());
        assert_eq!(l.light_type, point);
        assert_eq!(l.unif.is_set, 1);
        assert_eq!(l.unif.light_type, LightU::POINT);
        assert_eq!(l.unif.intensity, 500.0);
        match point {
            LightType::Point { range } => assert_eq!(l.unif.range, range),
            _ => unreachable!(),
        }
        assert_eq!(l.unif.color, YELLOW);
    }

    #[test]
    fn spot() {
        const BLUE: [f32; 3] = [0.0, 0.0, 1.0];

        let spot = LightType::Spot {
            range: 9.5,
            inner_angle: std::f32::consts::FRAC_PI_8,
            outer_angle: std::f32::consts::FRAC_PI_2,
        };

        let l = Light::new(spot, 650.0, BLUE.into());
        assert_eq!(l.light_type, spot);
        assert_eq!(l.unif.is_set, 1);
        assert_eq!(l.unif.light_type, LightU::SPOT);
        assert_eq!(l.unif.intensity, 650.0);
        match spot {
            LightType::Spot {
                range,
                inner_angle,
                outer_angle,
            } => {
                assert_eq!(l.unif.range, range);
                assert_eq!(
                    l.unif.angular_scale,
                    1.0 / (inner_angle.cos() - outer_angle.cos())
                );
                assert_eq!(
                    l.unif.angular_offset,
                    l.unif.angular_scale * -outer_angle.cos()
                );
            }
            _ => unreachable!(),
        }
        assert_eq!(l.unif.color, BLUE);
    }

    #[test]
    fn clamp_range() {
        for i in [
            LightType::Point { range: -1.0 },
            LightType::Spot {
                range: -0.25,
                inner_angle: 0.79,
                outer_angle: 1.57,
            },
        ] {
            let l = Light::new_white(i, 1000.0);
            assert!(
                match l.light_type() {
                    LightType::Point { range } => range,
                    LightType::Spot { range, .. } => range,
                    _ => unreachable!(),
                } > 0.0
            );
            assert!(l.unif.range > 0.0);
        }
    }

    #[test]
    fn clamp_angle() {
        for i in [
            LightType::Spot {
                range: 20.0,
                inner_angle: 0.0,
                outer_angle: std::f32::consts::PI,
            },
            LightType::Spot {
                range: 20.0,
                inner_angle: 0.0,
                outer_angle: 0.0,
            },
            LightType::Spot {
                range: 20.0,
                inner_angle: 1.57,
                outer_angle: 0.79,
            },
            LightType::Spot {
                range: 20.0,
                inner_angle: -0.79,
                outer_angle: 1.57,
            },
        ] {
            let l = Light::new_white(i, 10000.0);
            match l.light_type() {
                LightType::Spot {
                    inner_angle,
                    outer_angle,
                    ..
                } => {
                    assert!(inner_angle >= 0.0);
                    assert!(inner_angle < outer_angle);
                    assert!(outer_angle <= std::f32::consts::FRAC_PI_2);
                }
                _ => unreachable!(),
            }
            assert!(l.unif.angular_scale > 0.0);
            assert!(l.unif.angular_scale > l.unif.angular_offset);
        }
    }
}
