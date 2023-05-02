// Copyright 2022 Gustavo C. Viegas. All rights reserved.

//! Bounding shapes.

use crate::linear::{Mat3, Mat4, Vec3, Vec4};

/// Bounding box.
#[derive(Copy, Clone, PartialEq, Debug)]
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

    /// Transforms the bounding box.
    pub fn transform(self, xform: &Mat4<f32>) -> Self {
        let ul = Mat3::from(xform);
        let mut min = Vec3::from(xform[3]);
        let mut max = min;
        for i in 0..3 {
            for j in 0..3 {
                let d = (
                    ul[i][j] * (self.center[i] - self.half_extent[i]),
                    ul[i][j] * (self.center[i] + self.half_extent[i]),
                );
                if d.0 <= d.1 {
                    min[j] += d.0;
                    max[j] += d.1;
                } else {
                    min[j] += d.1;
                    max[j] += d.0;
                }
            }
        }
        Self {
            center: (min + max) / 2.0,
            half_extent: (max - min) / 2.0,
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

    /// Checks whether a bounding box contains a point.
    pub fn contains(&self, point: Vec3<f32>) -> bool {
        let min = self.center - self.half_extent;
        let max = self.center + self.half_extent;

        min[0] <= point[0]
            && max[0] >= point[0]
            && min[1] <= point[1]
            && max[1] >= point[1]
            && min[2] <= point[2]
            && max[2] >= point[2]
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
        let p = Vec3::new(
            sphere.center[0].clamp(min[0], max[0]),
            sphere.center[1].clamp(min[1], max[1]),
            sphere.center[2].clamp(min[2], max[2]),
        );

        (p - sphere.center).length() < sphere.radius
    }
}

impl From<Sphere> for Bbox {
    /// Converts from a `Sphere` into its enclosing `Bbox`.
    fn from(sphere: Sphere) -> Self {
        Self {
            center: sphere.center,
            half_extent: Vec3::from(sphere.radius),
        }
    }
}

/// Sphere.
#[derive(Copy, Clone, PartialEq, Debug)]
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

    /// Checks whether a sphere contains a point.
    pub fn contains(&self, point: Vec3<f32>) -> bool {
        (point - self.center).length() < self.radius
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

impl From<Bbox> for Sphere {
    /// Converts from a `Bbox` into its enclosing `Sphere`.
    fn from(bbox: Bbox) -> Self {
        Self {
            center: bbox.center,
            radius: bbox.half_extent.length(),
        }
    }
}

/// Infinite plane.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Plane {
    coef: Vec4<f32>,
}

impl Plane {
    /// Creates a new plane.
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        Self {
            coef: Vec4::new(a, b, c, d),
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
            coef: Vec4::new(n[0], n[1], n[2], -n.dot(&p0)),
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

    /// Checks whether a given point is on the plane.
    pub fn contains(&self, point: Vec3<f32>) -> bool {
        let n = self.n();
        let d = self.coef[3];
        // TODO: Disallow unnormalized planes.
        ((n.dot(&point) + d) / n.length()).abs() <= 1e-6
    }

    /// Computes the signed distance from the plane to a given point.
    pub fn signed_distance(&self, point: Vec3<f32>) -> f32 {
        let n = self.n();
        let d = self.coef[3];
        // TODO: Disallow unnormalized planes.
        (n.dot(&point) + d) / n.length()
    }
}

impl From<Vec4<f32>> for Plane {
    /// Converts from a `Vec4<f32>` containing the `[A, B, C, D]` coefficients
    /// of the `Plane`'s equation.
    fn from(coef: Vec4<f32>) -> Self {
        Self { coef }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bbox() {
        let bb0 = Bbox::new(Vec3::default(), Vec3::from(1.0));
        let bb1 = Bbox::new_origin(Vec3::from(1.0));
        assert_eq!(bb0, bb1);

        let off = Vec3::new(-1.0, 2.0, 0.25);
        assert_eq!(bb0.displace_by(off), bb1.displace_by(off));
        assert_eq!(bb0, bb1.displace_by(Vec3::default()));

        let bb0 = bb0.displace_by(off);
        assert_ne!(bb0, bb1);
        assert_eq!(bb0.displace_by(-off), bb1);
        assert_eq!(bb0.displace_by(-bb0.center()), bb1);

        let bb1 = bb1.displace_by(off);
        let off = Vec3::new(10.0, 42.0, 4.0);
        assert_eq!(bb0.resize_by(off), bb1.resize_by(off));
        assert_eq!(bb0.resize_by(Vec3::default()), bb1);

        let bb1 = bb1.resize_by(off);
        assert_ne!(bb0, bb1);
        assert_eq!(bb0, bb1.resize_by(-off));
        assert_eq!(bb0.resize_by(bb1.half_extent() - Vec3::from(1.0)), bb1);
    }

    #[test]
    fn sphere() {
        let sph0 = Sphere::new_origin(1.0);
        let sph1 = Sphere::new(Vec3::default(), 1.0);
        assert_eq!(sph0, sph1);

        let off = Vec3::new(2.0, -2.5, -0.01);
        assert_eq!(sph0.displace_by(off), sph1.displace_by(--off));
        assert_eq!(sph0.displace_by(-Vec3::default()), sph1);

        let sph1 = sph1.displace_by(-off);
        assert_ne!(sph0, sph1);
        assert_eq!(sph0, sph1.displace_by(off));
        assert_eq!(sph0.displace_by(sph1.center()), sph1);

        let sph0 = sph0.displace_by(-off);
        let off = 2.125;
        assert_eq!(sph0.resize_by(off), sph1.resize_by(off));
        assert_eq!(sph0.resize_by(0.0), sph1);

        let sph0 = sph0.resize_by(off);
        assert_ne!(sph0, sph1);
        assert_eq!(sph0, sph1.resize_by(off));
        assert_eq!(sph0, sph1.resize_by(sph0.radius - 1.0));
    }

    #[test]
    fn plane() {
        let pln0 = Plane::new(0.0, 1.0, 0.0, 0.0);
        let pln1 = Plane::from(Vec4::new(0.0, 1.0, 0.0, 0.0));
        let pln2 = Plane::from(-pln0.coef());
        assert_eq!(pln0, pln1);
        assert_ne!(pln0, pln2);
        assert_ne!(pln1, pln2);

        let pln0 = Plane::from(pln2.coef());
        assert_ne!(pln0, pln1);
        assert_eq!(pln0, pln2);
        assert_ne!(pln1, pln2);

        let pln0 = Plane::new_norm(Vec3::new(0.0, 1e9, 0.0), Vec3::default());
        let pln2 = Plane::new_unnorm(Vec3::new(0.0, 1.0, 0.0), Vec3::default());
        assert_eq!(pln0, pln1);
        assert_eq!(pln2, pln1);

        let n = Vec3::new(-2.0, 0.0, 0.0);
        let p0 = Vec3::new(1.0, -1.0, 0.0);
        let pln0 = Plane::new_unnorm(n, p0);
        let pln1 = Plane::new_norm(n, pln0.p0());
        let nn = pln0.n().norm();
        let pln2 = Plane::new(nn[0], nn[1], nn[2], -nn.dot(&pln1.p0()));
        assert_ne!(pln0, pln1);
        assert_ne!(pln0, pln2);
        assert_eq!(pln1, pln2);
    }

    #[test]
    fn bbox_transform() {
        let bb0 = Bbox::new_origin(Vec3::from(1.0));
        let bb = bb0.transform(&Mat4::from(1.0));
        assert_eq!(bb, bb0);

        let bb = bb0.transform(&Mat4::translation(2.0, -3.0, 0.5));
        assert_eq!(bb, bb0.displace_by(Vec3::new(2.0, -3.0, 0.5)));

        let bb = bb0.transform(&Mat4::from(2.0));
        assert_eq!(bb, bb0.resize_by(Vec3::from(1.0)));

        let t = Mat4::translation(3.0, 1.5, 0.75);
        let s = Mat4::scale(2.0, -0.5, 5.0);
        let bb = bb0.transform(&(t * s));
        assert_eq!(
            bb,
            bb0.displace_by(Vec3::new(3.0, 1.5, 0.75))
                .resize_by(Vec3::new(1.0, -0.5, 4.0))
        );

        let r = Mat4::rotation(std::f32::consts::FRAC_PI_2, &Vec3::new(1.0, -1.0, 1.0));
        let bb = bb0.transform(&r);
        assert_eq!(0.0, bb.center()[0]);
        assert_eq!(0.0, bb.center()[1]);
        assert_eq!(0.0, bb.center()[2]);
        assert_eq!(bb.half_extent()[0], bb.half_extent()[1]);
        assert_eq!(bb.half_extent()[1], bb.half_extent()[2]);
        assert!(bb.half_extent()[0] > 1.0);

        let r = Mat4::rotation_y(-std::f32::consts::FRAC_PI_2);
        let bb0 = bb0
            .displace_by(Vec3::new(-30.0, 20.0, 10.0))
            .resize_by(Vec3::new(3.0, 15.0, 63.0));
        let bb = bb0.transform(&r);
        assert!((-10.0 - bb.center()[0]).abs() < 1e-6);
        assert!((20.0 - bb.center()[1]).abs() < 1e-6);
        assert!((-30.0 - bb.center()[2]).abs() < 1e-6);
        assert!((64.0 - bb.half_extent()[0]).abs() < 1e-6);
        assert!((16.0 - bb.half_extent()[1]).abs() < 1e-6);
        // Notice the huge fp error here.
        assert!((4.0 - bb.half_extent()[2]).abs() < 3e-6);

        let r = r * Mat4::rotation_y(std::f32::consts::FRAC_PI_2);
        let bb = bb0.transform(&r);
        assert!((-30.0 - bb.center()[0]).abs() < 1e-6);
        assert!((20.0 - bb.center()[1]).abs() < 1e-6);
        assert!((10.0 - bb.center()[2]).abs() < 1e-6);
        assert!((4.0 - bb.half_extent()[0]).abs() < 1e-6);
        assert!((16.0 - bb.half_extent()[1]).abs() < 1e-6);
        assert!((64.0 - bb.half_extent()[2]).abs() < 1e-6);

        let m = Mat4::translation(-2.0, 3.0, -0.25)
            * Mat4::rotation_z(-std::f32::consts::PI)
            * Mat4::scale(1.2, 16.0, 10.1);
        let bb = Bbox::new_origin(Vec3::new(5.0, 6.0, 7.0)).transform(&m);
        assert!((-2.0 - bb.center()[0]).abs() < 1e-6);
        assert!((3.0 - bb.center()[1]).abs() < 1e-6);
        assert!((-0.25 - bb.center()[2]).abs() < 1e-6);
        // Notice the huge fp error here.
        assert!((1.2 * 5.0 - bb.half_extent()[0]).abs() < 9e-6);
        assert!((16.0 * 6.0 - bb.half_extent()[1]).abs() < 1e-6);
        assert!((10.1 * 7.0 - bb.half_extent()[2]).abs() < 1e-6);

        let m = Mat4::translation(-2.0, 3.0, -0.25)
            * Mat4::rotation_z(-std::f32::consts::FRAC_PI_2)
            * Mat4::scale(1.2, 16.0, 10.1);
        let bb = Bbox::new_origin(Vec3::new(5.0, 6.0, 7.0)).transform(&m);
        assert!((-2.0 - bb.center()[0]).abs() < 1e-6);
        assert!((3.0 - bb.center()[1]).abs() < 1e-6);
        assert!((-0.25 - bb.center()[2]).abs() < 1e-6);
        assert!((16.0 * 6.0 - bb.half_extent()[0]).abs() < 1e-6);
        // Notice the huge fp error here.
        assert!((1.2 * 5.0 - bb.half_extent()[1]).abs() < 4e-6);
        assert!((10.1 * 7.0 - bb.half_extent()[2]).abs() < 1e-6);
    }

    #[test]
    fn bbox_contains() {
        let bb0 = Bbox::new(Vec3::default(), Vec3::from(1.0));
        let bb = bb0;
        assert!(bb.contains(Vec3::default()));
        assert!(bb.contains(Vec3::from(1.0)));
        assert!(bb.contains(Vec3::from(-1.0)));
        assert!(bb.contains(Vec3::from(1.0)));
        assert!(bb.contains(Vec3::from(-1.0)));
        assert!(bb.contains(Vec3::new(0.5, 0.0, 0.0)));
        assert!(bb.contains(Vec3::new(0.0, -0.5, 0.0)));
        assert!(bb.contains(Vec3::new(0.0, 0.0, 0.5)));
        assert!(bb.contains(Vec3::new(-1.0, 0.0, 0.0)));
        assert!(bb.contains(Vec3::new(0.0, 1.0, 0.0)));
        assert!(bb.contains(Vec3::new(0.0, 0.0, -1.0)));
        assert!(bb.contains(Vec3::new(-1.0, 0.0, 1.0)));
        assert!(bb.contains(Vec3::new(1.0, -1.0, 0.0)));
        assert!(!bb.contains(Vec3::from(1.0001)));
        assert!(!bb.contains(Vec3::from(-1.0001)));
        assert!(!bb.contains(Vec3::new(0.0, 0.0, -2.5)));
        assert!(!bb.contains(Vec3::new(-1.0, 5.0, 1.0)));
        assert!(!bb.contains(Vec3::new(1.0, -1.0, 10.0)));

        let bb = bb0.displace_by(Vec3::from(2.1));
        assert!(!bb.contains(Vec3::default()));
        assert!(!bb.contains(Vec3::from(1.0)));
        assert!(!bb.contains(Vec3::from(-1.0)));
        assert!(bb.contains(Vec3::from(2.0)));
        assert!(!bb.contains(Vec3::from(-2.0)));
        assert!(!bb.contains(Vec3::new(0.5, 0.0, 0.0)));
        assert!(!bb.contains(Vec3::new(0.0, -0.5, 0.0)));
        assert!(!bb.contains(Vec3::new(0.0, 0.0, 0.5)));
        assert!(!bb.contains(Vec3::new(-1.5, 0.0, 0.0)));
        assert!(!bb.contains(Vec3::new(0.0, 1.5, 0.0)));
        assert!(!bb.contains(Vec3::new(0.0, 0.0, -1.5)));
        assert!(!bb.contains(Vec3::new(-1.0, 0.0, 1.0)));
        assert!(!bb.contains(Vec3::new(1.0, -1.0, 0.0)));
        assert!(bb.contains(Vec3::from(2.0001)));
        assert!(!bb.contains(Vec3::from(-2.0001)));
        assert!(!bb.contains(Vec3::new(0.0, 0.0, -2.5)));
        assert!(!bb.contains(Vec3::new(-1.0, 5.0, 1.0)));
        assert!(!bb.contains(Vec3::new(1.0, -1.0, 10.0)));

        let bb = bb0.resize_by(Vec3::from(1.0));
        assert!(bb.contains(Vec3::default()));
        assert!(bb.contains(Vec3::from(1.0)));
        assert!(bb.contains(Vec3::from(-1.0)));
        assert!(bb.contains(Vec3::from(2.0)));
        assert!(bb.contains(Vec3::from(-2.0)));
        assert!(bb.contains(Vec3::new(0.5, 0.0, 0.0)));
        assert!(bb.contains(Vec3::new(0.0, -0.5, 0.0)));
        assert!(bb.contains(Vec3::new(0.0, 0.0, 0.5)));
        assert!(bb.contains(Vec3::new(-1.5, 0.0, 0.0)));
        assert!(bb.contains(Vec3::new(0.0, 1.5, 0.0)));
        assert!(bb.contains(Vec3::new(0.0, 0.0, -1.5)));
        assert!(bb.contains(Vec3::new(-1.0, 0.0, 1.0)));
        assert!(bb.contains(Vec3::new(1.0, -1.0, 0.0)));
        assert!(!bb.contains(Vec3::from(2.0001)));
        assert!(!bb.contains(Vec3::from(-2.0001)));
        assert!(!bb.contains(Vec3::new(0.0, 0.0, -2.5)));
        assert!(!bb.contains(Vec3::new(-1.0, 5.0, 1.0)));
        assert!(!bb.contains(Vec3::new(1.0, -1.0, 10.0)));

        let bb = bb0
            .displace_by(Vec3::new(1.0, 0.0, 0.0))
            .resize_by(Vec3::new(-0.5, 2.5, 9.0));
        assert!(!bb.contains(Vec3::default()));
        assert!(bb.contains(Vec3::from(1.0)));
        assert!(!bb.contains(Vec3::from(-1.0)));
        assert!(!bb.contains(Vec3::from(2.0)));
        assert!(!bb.contains(Vec3::from(-2.0)));
        assert!(bb.contains(Vec3::new(0.5, 0.0, 0.0)));
        assert!(!bb.contains(Vec3::new(0.0, -0.5, 0.0)));
        assert!(!bb.contains(Vec3::new(0.0, 0.0, 0.5)));
        assert!(!bb.contains(Vec3::new(-1.5, 0.0, 0.0)));
        assert!(!bb.contains(Vec3::new(0.0, 1.5, 0.0)));
        assert!(!bb.contains(Vec3::new(0.0, 0.0, -1.5)));
        assert!(!bb.contains(Vec3::new(-1.0, 0.0, 1.0)));
        assert!(bb.contains(Vec3::new(1.0, -1.0, 0.0)));
        assert!(!bb.contains(Vec3::from(2.0001)));
        assert!(!bb.contains(Vec3::from(-2.0001)));
        assert!(!bb.contains(Vec3::new(0.0, 0.0, -2.5)));
        assert!(!bb.contains(Vec3::new(-1.0, 5.0, 1.0)));
        assert!(bb.contains(Vec3::new(1.0, -1.0, 10.0)));
    }

    #[test]
    fn sphere_contains() {
        let sph0 = Sphere::new(Vec3::default(), 1.0);
        let sph = sph0;
        assert!(sph.contains(Vec3::default()));
        assert!(!sph.contains(Vec3::from(1.0)));
        assert!(!sph.contains(Vec3::from(-1.0)));
        assert!(sph.contains(Vec3::from(1.0).norm()));
        assert!(sph.contains(Vec3::from(-1.0).norm()));
        assert!(sph.contains(Vec3::new(1.0 / 3f32.sqrt(), 0.0, 0.0)));
        assert!(sph.contains(Vec3::new(0.0, -1.0 / 3f32.sqrt(), 0.0)));
        assert!(sph.contains(Vec3::new(0.0, 0.0, -1.0 / 3f32.sqrt())));
        assert!(sph.contains(Vec3::new(-0.25, 0.3333, 0.0)));
        assert!(sph.contains(Vec3::new(0.0, -0.3333, 0.25)));

        let sph = sph0.displace_by(Vec3::new(0.0, -1.0, 0.0));
        assert!(!sph.contains(Vec3::default()));
        assert!(!sph.contains(Vec3::from(1.0)));
        assert!(!sph.contains(Vec3::from(-1.0)));
        assert!(!sph.contains(Vec3::from(1.0).norm()));
        assert!(sph.contains(Vec3::from(-1.0).norm()));
        assert!(!sph.contains(Vec3::new(1.0 / 3f32.sqrt(), 0.0, 0.0)));
        assert!(sph.contains(Vec3::new(0.0, -1.0 / 3f32.sqrt(), 0.0)));
        assert!(!sph.contains(Vec3::new(0.0, 0.0, -1.0 / 3f32.sqrt())));
        assert!(!sph.contains(Vec3::new(-0.25, 0.3333, 0.0)));
        assert!(sph.contains(Vec3::new(0.0, -0.3333, 0.25)));

        let sph = sph0.resize_by(1.0);
        assert!(sph.contains(Vec3::default()));
        assert!(sph.contains(Vec3::from(1.0)));
        assert!(sph.contains(Vec3::from(-1.0)));
        assert!(sph.contains(Vec3::from(1.0).norm()));
        assert!(sph.contains(Vec3::from(-1.0).norm()));
        assert!(sph.contains(Vec3::new(1.0 / 3f32.sqrt(), 0.0, 0.0)));
        assert!(sph.contains(Vec3::new(0.0, -1.0 / 3f32.sqrt(), 0.0)));
        assert!(sph.contains(Vec3::new(0.0, 0.0, -1.0 / 3f32.sqrt())));
        assert!(sph.contains(Vec3::new(-0.25, 0.3333, 0.0)));
        assert!(sph.contains(Vec3::new(0.0, -0.3333, 0.25)));

        let sph = sph0
            .displace_by(Vec3::from(0.75).norm())
            .resize_by(0.817 - 1.0);
        assert!(!sph.contains(Vec3::default()));
        assert!(sph.contains(Vec3::from(1.0)));
        assert!(!sph.contains(Vec3::from(-1.0)));
        assert!(sph.contains(Vec3::from(1.0).norm()));
        assert!(!sph.contains(Vec3::from(-1.0).norm()));
        assert!(sph.contains(Vec3::new(1.0 / 3f32.sqrt(), 0.0, 0.0)));
        assert!(!sph.contains(Vec3::new(0.0, -1.0 / 3f32.sqrt(), 0.0)));
        assert!(!sph.contains(Vec3::new(0.0, 0.0, -1.0 / 3f32.sqrt())));
        assert!(!sph.contains(Vec3::new(-0.25, 0.3333, 0.0)));
        assert!(!sph.contains(Vec3::new(0.0, -0.3333, 0.25)));
    }

    #[test]
    fn plane_contains() {
        let pln = Plane::new(0.0, 1.0, 0.0, 0.0);
        assert!(pln.contains(Vec3::default()));
        assert!(pln.contains(pln.p0()));
        assert!(pln.contains(pln.p0() + Vec3::new(-1e9, 0.0, 1e9)));
        assert!(pln.contains(pln.p0() + Vec3::new(0.0, 0.0, -1e9)));
        assert!(pln.contains(pln.p0() + Vec3::new(1e9, 0.0, 0.0)));
        assert!(!pln.contains(pln.p0() + Vec3::new(0.0, 0.0001, 0.0)));
        assert!(!pln.contains(pln.p0() + Vec3::new(0.0, 1.0, 0.0)));
        assert!(!pln.contains(pln.p0() + Vec3::new(0.0, -1.0, 0.0)));
        assert!(!pln.contains(pln.p0() + Vec3::new(0.0, 1.0, -1.0)));
        assert!(!pln.contains(pln.p0() + Vec3::new(-1.0, 1.0, 0.0)));
        assert!(!pln.contains(pln.p0() + Vec3::from(1.0)));

        let pln = Plane::new_norm(Vec3::new(0.0, 1.0, -1.0), Vec3::default());
        assert!(pln.contains(Vec3::default()));
        assert!(pln.contains(pln.p0()));
        assert!(!pln.contains(pln.p0() + Vec3::new(-1e9, 0.0, 1e9)));
        assert!(!pln.contains(pln.p0() + Vec3::new(0.0, 0.0, -1e9)));
        assert!(pln.contains(pln.p0() + Vec3::new(1e9, 0.0, 0.0)));
        assert!(!pln.contains(pln.p0() + Vec3::new(0.0, 0.0001, 0.0)));
        assert!(!pln.contains(pln.p0() + Vec3::new(0.0, 1.0, 0.0)));
        assert!(!pln.contains(pln.p0() + Vec3::new(0.0, -1.0, 0.0)));
        assert!(!pln.contains(pln.p0() + Vec3::new(0.0, 1.0, -1.0)));
        assert!(!pln.contains(pln.p0() + Vec3::new(-1.0, 1.0, 0.0)));
        assert!(pln.contains(pln.p0() + Vec3::from(1.0)));

        let mut v = Vec4::new(1.0, 1.0, 0.0, 0.0).norm();
        v[3] = -Vec3::from(v).dot(&Vec3::from(2.0));
        let pln = Plane::from(v);
        assert!(!pln.contains(Vec3::default()));
        assert!(pln.contains(pln.p0()));
        assert!(!pln.contains(Vec3::new(-1e9, 0.0, 1e9)));
        assert!(!pln.contains(Vec3::new(0.0, 0.0, -1e9)));
        assert!(!pln.contains(Vec3::new(1e9, 0.0, 0.0)));
        assert!(!pln.contains(Vec3::new(0.0, 0.0001, 0.0)));
        assert!(!pln.contains(Vec3::new(0.0, 1.0, 0.0)));
        assert!(!pln.contains(Vec3::new(0.0, -1.0, 0.0)));
        assert!(!pln.contains(Vec3::new(0.0, 1.0, -1.0)));
        assert!(!pln.contains(Vec3::new(-1.0, 1.0, 0.0)));
        assert!(!pln.contains(Vec3::from(1.0)));
        assert!(pln.contains(Vec3::new(pln.p0()[0], pln.p0()[1], 0.0)));
        assert!(pln.contains(Vec3::new(pln.p0()[0], pln.p0()[1], -14.0)));
        assert!(pln.contains(Vec3::new(pln.p0()[0], pln.p0()[1], 123.0)));
        assert!(!pln.contains(Vec3::new(pln.p0()[0] + 1.0, pln.p0()[1], pln.p0()[2])));
        assert!(!pln.contains(Vec3::new(pln.p0()[0], pln.p0()[1] + 1.0, pln.p0()[2])));
        assert!(!pln.contains(Vec3::new(pln.p0()[0] + 1.0, pln.p0()[1] + 1.0, pln.p0()[2])));
    }

    #[test]
    fn bbox_intersects() {
        let bb0 = Bbox::new_origin(Vec3::from(1.0));
        let bb1 = bb0;
        assert!(bb0.intersects(bb1));
        assert!(bb1.intersects(bb0));

        let off = Vec3::from(1.0);
        assert!(bb0.intersects(bb1.displace_by(off)));
        assert!(bb0.displace_by(off).intersects(bb1));

        let off = Vec3::from(-1.0);
        assert!(bb0.intersects(bb1.displace_by(off)));
        assert!(bb0.displace_by(off).intersects(bb1));

        let off = Vec3::from(2.0);
        assert!(bb0.intersects(bb1.displace_by(off)));
        assert!(bb0.displace_by(off).intersects(bb1));

        let off = Vec3::from(-2.0);
        assert!(bb0.intersects(bb1.displace_by(off)));
        assert!(bb0.displace_by(off).intersects(bb1));

        let off = Vec3::from(2.01);
        assert!(!bb0.intersects(bb1.displace_by(off)));
        assert!(!bb0.displace_by(off).intersects(bb1));

        let off = Vec3::from(-2.01);
        assert!(!bb0.intersects(bb1.displace_by(off)));
        assert!(!bb0.displace_by(off).intersects(bb1));

        let off = Vec3::new(1.0, 0.0, 0.0);
        assert!(bb0.intersects(bb1.displace_by(off)));
        assert!(bb0.displace_by(off).intersects(bb1));

        let off = Vec3::new(0.0, -1.25, 0.2);
        assert!(bb0.intersects(bb1.displace_by(off)));
        assert!(bb0.displace_by(off).intersects(bb1));

        let off = Vec3::new(0.0, 0.0, 2.01);
        assert!(!bb0.intersects(bb1.displace_by(off)));
        assert!(!bb0.displace_by(off).intersects(bb1));

        let off = Vec3::from(1.0);
        assert!(bb0.intersects(bb1.resize_by(off)));
        assert!(bb0.resize_by(off).intersects(bb1));

        let off = Vec3::from(-1.0);
        assert!(bb0.intersects(bb1.resize_by(off)));
        assert!(bb0.resize_by(off).intersects(bb1));

        let off = Vec3::new(0.0, -1.0, 1.0);
        assert!(bb0.intersects(bb1.resize_by(off)));
        assert!(bb0.resize_by(off).intersects(bb1));

        let off = Vec3::new(10.0, 20.0, 30.0);
        assert!(bb0.intersects(bb1.resize_by(off)));
        assert!(bb0.resize_by(off).intersects(bb1));

        let d = Vec3::new(1.0, 0.0, 0.0);
        let r = Vec3::new(0.0, -1.01, 0.0);
        assert!(bb0.displace_by(d).intersects(bb1.resize_by(r)));
        assert!(bb0.resize_by(r).intersects(bb1.displace_by(d)));

        let d = Vec3::new(0.0, 0.0, -1.0);
        let r = Vec3::new(0.0, 0.0, -1.01);
        assert!(!bb0.displace_by(d).intersects(bb1.resize_by(r)));
        assert!(!bb0.resize_by(r).intersects(bb1.displace_by(d)));
    }

    #[test]
    fn sphere_intersects() {
        let sph0 = Sphere::new_origin(1.0);
        let sph1 = sph0;
        assert!(sph0.intersects(sph1));
        assert!(sph1.intersects(sph0));

        let off = Vec3::from(1.0);
        assert!(sph0.intersects(sph1.displace_by(off)));
        assert!(sph0.displace_by(off).intersects(sph1));

        let off = Vec3::from(-1.0);
        assert!(sph0.intersects(sph1.displace_by(off)));
        assert!(sph0.displace_by(off).intersects(sph1));

        let off = Vec3::new(0.0, 1.0, 0.0);
        assert!(sph0.intersects(sph1.displace_by(off)));
        assert!(sph0.displace_by(off).intersects(sph1));

        let off = Vec3::new(0.0, -1.25, -0.2);
        assert!(sph0.intersects(sph1.displace_by(off)));
        assert!(sph0.displace_by(off).intersects(sph1));

        // TODO: Shouldn't we do the distance check using
        // less equal instead?
        let off = Vec3::new(-1.999999, 0.0, 0.0);
        assert!(sph0.intersects(sph1.displace_by(off)));
        assert!(sph0.displace_by(off).intersects(sph1));

        // ... so this one would intersect.
        let off = Vec3::new(0.0, 0.0, 2.0);
        assert!(!sph0.intersects(sph1.displace_by(off)));
        assert!(!sph0.displace_by(off).intersects(sph1));

        let off = 20.0;
        assert!(sph0.intersects(sph1.resize_by(off)));
        assert!(sph0.resize_by(off).intersects(sph1));

        let off = -0.5;
        assert!(sph0.intersects(sph1.resize_by(off)));
        assert!(sph0.resize_by(off).intersects(sph1));

        let off = -1.0;
        assert!(sph0.intersects(sph1.resize_by(off)));
        assert!(sph0.resize_by(off).intersects(sph1));

        let d = Vec3::new(2.0, 0.0, 0.0);
        let r = 0.1;
        assert!(sph0.displace_by(d).intersects(sph1.resize_by(r)));
        assert!(sph0.resize_by(r).intersects(sph1.displace_by(d)));

        let d = Vec3::new(0.0, 0.0, -3.0);
        let r = 1.0;
        assert!(!sph0.displace_by(d).intersects(sph1.resize_by(r)));
        assert!(!sph0.resize_by(r).intersects(sph1.displace_by(d)));
    }

    #[test]
    fn bbox_sphere_intersects() {
        let bb = Bbox::new_origin(Vec3::from(1.0));
        let sph = Sphere::new_origin(1.0);
        assert!(bb.intersects_sphere(sph));
        assert!(sph.intersects_bbox(bb));

        let off = Vec3::new(1.0, -1.0, 0.0);
        assert!(bb.intersects_sphere(sph.displace_by(off)));
        assert!(bb.displace_by(off).intersects_sphere(sph));

        let off = Vec3::new(2.0, 0.0, 0.0);
        assert!(!bb.intersects_sphere(sph.displace_by(off)));
        assert!(!bb.displace_by(off).intersects_sphere(sph));

        let off = Vec3::from(2.0);
        assert!(bb.intersects_sphere(sph.resize_by(off[0])));
        assert!(bb.resize_by(off).intersects_sphere(sph));

        let off = Vec3::from(-1.0);
        assert!(!bb.intersects_sphere(sph.resize_by(off[0])));
        assert!(bb.resize_by(off).intersects_sphere(sph));

        let off = Vec3::from(-0.999999);
        assert!(bb.intersects_sphere(sph.resize_by(off[0])));
        assert!(bb.resize_by(off).intersects_sphere(sph));

        let d = Vec3::new(3.0, 0.0, 0.0);
        let r = Vec3::new(2.0, 0.0, 0.0);
        assert!(bb.displace_by(d).intersects_sphere(sph.resize_by(r[0])));
        assert!(bb.resize_by(r).intersects_sphere(sph.displace_by(d)));

        let d = Vec3::new(0.0, 1.5, 0.0);
        let r = Vec3::new(0.0, -0.5, 0.0);
        assert!(!bb.displace_by(d).intersects_sphere(sph.resize_by(r[1])));
        assert!(!bb.resize_by(r).intersects_sphere(sph.displace_by(d)));

        let d = Vec3::new(0.0, 0.0, 7.0);
        let r = Vec3::new(0.0, 0.0, 5.000001);
        assert!(bb.displace_by(d).intersects_sphere(sph.resize_by(r[2])));
        assert!(bb.resize_by(r).intersects_sphere(sph.displace_by(d)));
    }

    #[test]
    fn plane_signed_distance() {
        let pln = Plane::new(0.0, 1.0, 0.0, 0.0);
        assert_eq!(0.0, pln.signed_distance(Vec3::from(0.0)));
        assert_eq!(3.0, pln.signed_distance(Vec3::from(3.0)));
        assert_eq!(-3.0, pln.signed_distance(Vec3::from(-3.0)));
        assert_eq!(2.5, pln.signed_distance(Vec3::new(0.0, 2.5, 0.0)));
        assert_eq!(-2.5, pln.signed_distance(Vec3::new(0.0, -2.5, 0.0)));
        assert_eq!(0.0, pln.signed_distance(Vec3::new(-16.0, 0.0, 4.0)));
        assert_eq!(0.0, pln.signed_distance(Vec3::new(-16.0, 0.0, 4.0)));
        assert_eq!(0.0, pln.signed_distance(Vec3::new(1.25, 0.0, 0.0)));
        assert_eq!(0.0, pln.signed_distance(Vec3::new(0.0, 0.0, 11.1)));

        let pln = Plane::new_norm(Vec3::new(-1.0, 0.0, 1.0), Vec3::default());
        assert_eq!(0.0, pln.signed_distance(Vec3::from(0.0)));
        assert_eq!(0.0, pln.signed_distance(Vec3::from(1.0)));
        assert_eq!(0.0, pln.signed_distance(Vec3::from(-1.0)));
        assert!((pln.n()[2] - pln.signed_distance(Vec3::new(0.0, 0.0, 1.0))).abs() <= 1e-6);
        assert!((pln.n()[0] - pln.signed_distance(Vec3::new(1.0, 0.0, 0.0))).abs() <= 1e-6);
        assert_eq!(0.0, pln.signed_distance(Vec3::new(0.0, 1.0, 0.0)));

        let pln = Plane::new_norm(Vec3::new(-1.0, 0.0, 1.0), Vec3::new(2.0, 0.0, -2.0));
        assert!((pln.coef()[3] - pln.signed_distance(Vec3::from(0.0)).abs()) <= 1e-6);
        assert!(pln.signed_distance(pln.p0()).abs() <= 1e-6);
        assert!(pln.signed_distance(Vec3::from(0.0)) > 0.0);
        assert!(pln.signed_distance(Vec3::new(-2.0, 0.0, 2.0)) > 0.0);
        assert!(pln.signed_distance(Vec3::new(2.0001, 0.0, -2.0001)) < 0.0);
        assert!(
            pln.signed_distance(Vec3::new(pln.p0()[0], -1e9, pln.p0()[2]))
                .abs()
                <= 1e-6
        );
        assert!(
            pln.signed_distance(Vec3::new(pln.p0()[0], 1e9, pln.p0()[2]))
                .abs()
                <= 1e-6
        );
    }

    #[test]
    fn bbox_sphere_from() {
        let bb = Bbox::new_origin(Vec3::from(1.0));
        let sph = Sphere::from(bb);
        assert!((sph.radius() - 3f32.sqrt()).abs() <= f32::EPSILON);
        assert_eq!(sph.center()[0], bb.center()[0]);
        assert_eq!(sph.center()[1], bb.center()[1]);
        assert_eq!(sph.center()[2], bb.center()[2]);
        let bb = Bbox::from(sph);
        assert_eq!(bb.half_extent()[0], sph.radius());
        assert_eq!(bb.half_extent()[0], bb.half_extent()[1]);
        assert_eq!(bb.half_extent()[1], bb.half_extent()[2]);
        assert_eq!(bb.center()[0], sph.center()[0]);
        assert_eq!(bb.center()[1], sph.center()[1]);
        assert_eq!(bb.center()[2], sph.center()[2]);

        let sph = Sphere::new(Vec3::new(2.0, -1.0, 0.5), 2.0);
        let bb = Bbox::from(sph);
        assert_eq!(bb.half_extent()[0], sph.radius());
        assert_eq!(bb.half_extent()[0], bb.half_extent()[1]);
        assert_eq!(bb.half_extent()[1], bb.half_extent()[2]);
        assert_eq!(bb.center()[0], sph.center()[0]);
        assert_eq!(bb.center()[1], sph.center()[1]);
        assert_eq!(bb.center()[2], sph.center()[2]);
        let sph = Sphere::from(bb);
        assert!((sph.radius() - 12f32.sqrt()).abs() <= f32::EPSILON);
        assert_eq!(sph.center()[0], bb.center()[0]);
        assert_eq!(sph.center()[1], bb.center()[1]);
        assert_eq!(sph.center()[2], bb.center()[2]);

        let bb = Bbox::new(Vec3::from(-100.0), Vec3::new(1.0, 2.0, 3.0));
        let sph = Sphere::from(bb);
        assert!((sph.radius() - bb.half_extent.length()).abs() <= f32::EPSILON);
        assert_eq!(sph.center()[0], bb.center()[0]);
        assert_eq!(sph.center()[1], bb.center()[1]);
        assert_eq!(sph.center()[2], bb.center()[2]);
        let bb = Bbox::from(sph);
        assert_eq!(bb.half_extent()[0], sph.radius());
        assert_eq!(bb.half_extent()[0], bb.half_extent()[1]);
        assert_eq!(bb.half_extent()[1], bb.half_extent()[2]);
        assert_eq!(bb.center()[0], sph.center()[0]);
        assert_eq!(bb.center()[1], sph.center()[1]);
        assert_eq!(bb.center()[2], sph.center()[2]);
    }
}
