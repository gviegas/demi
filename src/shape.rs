// Copyright 2022 Gustavo C. Viegas. All rights reserved.

//! Bounding shapes.

use crate::linear::{Vec3, Vec4};

/// Bounding box.
#[derive(Copy, Clone, Debug)]
pub struct Bbox {
    center: Vec3<f32>,
    half_extent: Vec3<f32>,
}

impl Bbox {
    /// Creates a new bounding box.
    pub fn new(center: Vec3<f32>, half_extent: Vec3<f32>) -> Self {
        Self {
            center,
            half_extent,
        }
    }

    /// Creates a new bounding box centered at the origin.
    pub fn new_origin(half_extent: Vec3<f32>) -> Self {
        Self::new(Vec3::default(), half_extent)
    }

    /// Offsets the bounding box's center.
    pub fn offset(self, off: Vec3<f32>) -> Self {
        Self {
            center: self.center + off,
            ..self
        }
    }

    /// Returns the center.
    pub fn center(&self) -> Vec3<f32> {
        self.center
    }

    /// Returns the half extent.
    pub fn half_extent(&self) -> Vec3<f32> {
        self.half_extent
    }
}

/// Sphere.
#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    center: Vec3<f32>,
    radius: f32,
}

impl Sphere {
    /// Creates a new sphere.
    pub fn new(center: Vec3<f32>, radius: f32) -> Self {
        Self { center, radius }
    }

    /// Creates a new sphere centered at the origin.
    pub fn new_origin(radius: f32) -> Self {
        Self::new(Vec3::default(), radius)
    }

    /// Offsets the sphere's center.
    pub fn offset(self, off: Vec3<f32>) -> Self {
        Self {
            center: self.center + off,
            ..self
        }
    }

    /// Returns the center.
    pub fn center(&self) -> Vec3<f32> {
        self.center
    }

    /// Returns the radius.
    pub fn radius(&self) -> f32 {
        self.radius
    }
}

/// Infinite plane.
#[derive(Copy, Clone, Debug)]
pub struct Plane {
    coef: Vec4<f32>,
}

impl Plane {
    /// Creates a new plane.
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        Self {
            coef: Vec4::new([a, b, c, d]),
        }
    }

    /// Returns the coefficients of the plane's equation.
    pub fn coef(&self) -> Vec4<f32> {
        self.coef
    }
}
