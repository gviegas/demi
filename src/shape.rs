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

    /// Checks whether a bounding box intersects another.
    pub fn intersects(&self, other: Self) -> bool {
        let min0 = self.center - self.half_extent;
        let max0 = self.center + self.half_extent;
        let min1 = other.center - other.half_extent;
        let max1 = other.center + other.half_extent;

        min0[0] <= max1[0]
            && max0[0] >= min1[0]
            && min0[1] <= max1[1]
            && max0[1] >= min1[1]
            && min0[2] <= max1[2]
            && max0[2] >= min1[2]
    }

    /// Checks whether a bounding box intersects a sphere.
    pub fn intersects_sphere(&self, sphere: Sphere) -> bool {
        let min = self.center - self.half_extent;
        let max = self.center + self.half_extent;
        let p = Vec3::new([
            sphere.center[0].clamp(min[0], max[0]),
            sphere.center[1].clamp(min[1], max[1]),
            sphere.center[2].clamp(min[2], max[2]),
        ]);

        (p - sphere.center).length() < sphere.radius
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

    /// Checks whether a sphere intersects another.
    pub fn intersects(&self, other: Sphere) -> bool {
        (other.center - self.center).length() < other.radius + self.radius
    }

    /// Checks whether a sphere intersects a bounding box.
    pub fn intersects_bbox(&self, bbox: Bbox) -> bool {
        bbox.intersects_sphere(*self)
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
