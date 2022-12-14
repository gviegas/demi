// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use crate::linear::{Vec3, Vec4};
use crate::shape::{Bbox, Plane, Sphere};

impl PartialEq for Bbox {
    fn eq(&self, other: &Bbox) -> bool {
        let c = self.center - other.center;
        let e = self.half_extent - other.half_extent;
        c[0].abs() <= f32::EPSILON
            && c[1].abs() <= f32::EPSILON
            && c[2].abs() <= f32::EPSILON
            && e[0].abs() <= f32::EPSILON
            && e[1].abs() <= f32::EPSILON
            && e[2].abs() <= f32::EPSILON
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Sphere) -> bool {
        let c = self.center - other.center;
        let r = self.radius - other.radius;
        c[0].abs() <= f32::EPSILON
            && c[1].abs() <= f32::EPSILON
            && c[2].abs() <= f32::EPSILON
            && r.abs() <= f32::EPSILON
    }
}

impl PartialEq for Plane {
    fn eq(&self, other: &Plane) -> bool {
        let c = self.coef - other.coef;
        c[0].abs() <= f32::EPSILON
            && c[1].abs() <= f32::EPSILON
            && c[2].abs() <= f32::EPSILON
            && c[3].abs() <= f32::EPSILON
    }
}

#[test]
fn bbox() {
    let bb0 = Bbox::new(Vec3::new([0.0; 3]), Vec3::new([1.0; 3]));
    let bb1 = Bbox::new_origin(Vec3::from(1.0));
    assert_eq!(bb0, bb1);

    let off = Vec3::new([-1.0, 2.0, 0.25]);
    assert_eq!(bb0.displace_by(off), bb1.displace_by(off));
    assert_eq!(bb0, bb1.displace_by(Vec3::default()));

    let bb0 = bb0.displace_by(off);
    assert_ne!(bb0, bb1);
    assert_eq!(bb0.displace_by(-off), bb1);
    assert_eq!(bb0.displace_by(-bb0.center()), bb1);

    let bb1 = bb1.displace_by(off);
    let off = Vec3::new([10.0, 42.0, 4.0]);
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

    let off = Vec3::new([2.0, -2.5, -0.01]);
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
    let pln1 = Plane::from(Vec4::new([0.0, 1.0, 0.0, 0.0]));
    let pln2 = Plane::from(-pln0.coef());
    assert_eq!(pln0, pln1);
    assert_ne!(pln0, pln2);
    assert_ne!(pln1, pln2);

    let pln0 = Plane::from(pln2.coef());
    assert_ne!(pln0, pln1);
    assert_eq!(pln0, pln2);
    assert_ne!(pln1, pln2);
}
