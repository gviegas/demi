// Copyright 2022 Gustavo C. Viegas. All rights reserved.

//! Bounding shapes.

#[cfg(test)]
mod tests;

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

    /// Displaces the bounding box by offsetting its center.
    pub fn displace_by(self, offset: Vec3<f32>) -> Self {
        Self {
            center: self.center + offset,
            ..self
        }
    }

    /// Resizes the bounding box by offsetting its half extent.
    pub fn resize_by(self, offset: Vec3<f32>) -> Self {
        Self {
            half_extent: self.half_extent + offset,
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

    /// Displaces the sphere by offsetting its center.
    pub fn displace_by(self, offset: Vec3<f32>) -> Self {
        Self {
            center: self.center + offset,
            ..self
        }
    }

    /// Resizes the sphere by offsetting its radius.
    pub fn resize_by(self, offset: f32) -> Self {
        Self {
            radius: self.radius + offset,
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

    /// Creates a new plane from a normal vector and a point on the plane.
    ///
    /// This function normalizes `n`.
    pub fn new_norm(n: Vec3<f32>, p0: Vec3<f32>) -> Self {
        Self::new_unnorm(n.norm(), p0)
    }

    /// Creates a new plane from a normal vector and a point on the plane.
    ///
    /// This function does *not* normalizes `n`.
    pub fn new_unnorm(n: Vec3<f32>, p0: Vec3<f32>) -> Self {
        Self {
            coef: Vec4::new([n[0], n[1], n[2], -n.dot(&p0)]),
        }
    }

    /// Returns the coefficients of the plane's equation.
    pub fn coef(&self) -> Vec4<f32> {
        self.coef
    }

    /// Returns the (possibly unnormalized) normal to the plane.
    pub fn n(&self) -> Vec3<f32> {
        Vec3::from(self.coef)
    }

    /// Returns the point on the plane that lies closest to the origin.
    pub fn p0(&self) -> Vec3<f32> {
        -self.n() * self.coef[3]
    }
}

impl From<Vec4<f32>> for Plane {
    /// Converts from a `Vec4<f32>` containing the `[A, B, C, D]` coefficients
    /// of the `Plane`'s equation.
    fn from(coef: Vec4<f32>) -> Self {
        Self { coef }
    }
}
