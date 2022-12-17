// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use crate::linear::{Mat4, Vec3, Vec4};
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
