// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use crate::linear::{Vec2, Vec3, Vec4};

#[test]
fn vec_index() {
    let a = [-1, 0, 2047];
    let v = Vec3::new(&a);
    for i in a.iter().enumerate() {
        assert_eq!(*i.1, v[i.0]);
    }
    let mut v = Vec3::new(&[0; 3]);
    v[1] = a[1];
    v[0] = a[2];
    v[2] = a[0];
    for i in a.iter().rev().enumerate() {
        assert_eq!(*i.1, v[i.0]);
    }
}

#[test]
fn vec_add() {
    let v = Vec4::new(&[1i8; 4]);
    let u = Vec4::new(&[-2i8, -3, 0, 1]);
    let w = &v + &u;
    let x = &u + &v;
    for i in 0..4 {
        assert_eq!(w[i], v[i] + u[i]);
        assert_eq!(w[i], x[i]);
    }
}

#[test]
fn vec_add_assign() {
    let mut v = Vec2::new(&[4, 2]);
    let u = Vec2::new(&[-10, 10]);
    let w = v;
    v += &u;
    for i in 0..2 {
        assert_eq!(v[i], w[i] + u[i]);
    }
}

#[test]
fn vec_sub() {
    let v = Vec4::new(&[1i8; 4]);
    let u = Vec4::new(&[-2i8, 3, 0, -1]);
    let w = &v - &u;
    let x = &u - &v;
    for i in 0..4 {
        assert_eq!(w[i], v[i] - u[i]);
        assert_ne!(w[i], x[i]);
        assert_eq!(x[i], u[i] - v[i]);
    }
}

#[test]
fn vec_sub_assign() {
    let mut v = Vec2::new(&[4, 2]);
    let u = Vec2::new(&[-10, 10]);
    let w = v;
    v -= &u;
    for i in 0..2 {
        assert_eq!(v[i], w[i] - u[i]);
    }
}

#[test]
fn vec_mul() {
    let a = [1u64, 99, 65535];
    let v = Vec3::new(&a);
    let s = 4096;
    let v = &v * s;
    for i in 0..3 {
        assert_eq!(v[i], a[i] * s);
    }
    let v = &v * 0;
    for i in 0..3 {
        assert_eq!(v[i], 0);
    }
}

#[test]
fn vec_mul_assign() {
    let mut v = Vec3::new(&[-1i64, -255, 256]);
    let s = -2;
    let w = v;
    v *= s;
    for i in 0..3 {
        assert_eq!(v[i], w[i] * s);
    }
    v *= 0;
    for i in 0..3 {
        assert_eq!(v[i], 0);
    }
}

#[test]
fn vec_div() {
    let a = [1u64, 99, 65535];
    let v = Vec3::new(&a);
    let s = 9;
    let v = &v / s;
    for i in 0..3 {
        assert_eq!(v[i], a[i] / s);
    }
    let w = v;
    let v = &v / 1;
    for i in 0..3 {
        assert_eq!(v[i], w[i]);
    }
}

#[test]
fn vec_div_assign() {
    let mut v = Vec3::new(&[-1i64, -255, 256]);
    let s = -2;
    let w = v;
    v /= s;
    for i in 0..3 {
        assert_eq!(v[i], w[i] / s);
    }
    let w = v;
    v /= 1;
    for i in 0..3 {
        assert_eq!(v[i], w[i]);
    }
}

#[test]
fn vec_neg() {
    let v = Vec4::new(&[-0.5, -1.0, 0.0, 0.125]);
    let u = -&v;
    for i in 0..4 {
        assert_eq!(v[i], -u[i]);
    }
    let u = -&u;
    for i in 0..4 {
        assert_eq!(v[i], u[i]);
    }
}

#[test]
fn vec_dot() {
    let v = Vec3::new(&[0f32, 0.7071068, 0.7071068]);
    let s = v.dot(&v);
    assert_eq!(s.signum(), 1.0);
    assert_eq!(s.round(), 1.0);
    let u = -&v;
    let s = v.dot(&u);
    assert_eq!(s.signum(), -1.0);
    assert_eq!(s.round(), -1.0);
}

#[test]
fn vec_length() {
    let v = Vec2::new(&[4f32, 3f32]);
    let s = v.length();
    assert_eq!(s, 5.0);
    let v = Vec4::new(&[4f32 / s, 3f32 / s, 0.0, 0.0]);
    let s = v.length();
    assert_eq!(s, 1.0);
}

#[test]
fn vec_norm() {
    let v = Vec3::new(&[3f64, 0.0, 4f64]);
    let u = v.norm();
    assert_eq!(u[0], 0.6);
    assert_eq!(u[1], 0.0);
    assert_eq!(u[2], 0.8);
    assert_eq!(u.length(), 1.0);
}

#[test]
fn vec_cross() {
    let v = Vec3::<f32>::new(&[0.0, 0.0, 1.0]);
    let u = Vec3::<f32>::new(&[0.0, 1.0, 0.0]);
    let w = v.cross(&u);
    assert_eq!(w[0], -1.0);
    assert_eq!(w[1], 0.0);
    assert_eq!(w[2], 0.0);
    let w = u.cross(&v);
    assert_eq!(w[0], 1.0);
    assert_eq!(w[1], 0.0);
    assert_eq!(w[2], 0.0);
    let w = v.cross(&-&v);
    assert_eq!(w[0], 0.0);
    assert_eq!(w[1], 0.0);
    assert_eq!(w[2], 0.0);
}
