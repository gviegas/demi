// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use crate::linear::{Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec4};

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

#[test]
fn mat_index() {
    let a = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let m = Mat3::new(&a);
    let mut n = Mat3::<i32>::default();
    n[0] = m[2];
    n[1] = m[1];
    n[2] = m[0];
    for i in a.into_iter().rev().enumerate() {
        for j in i.1.into_iter().enumerate() {
            assert_eq!(j.1, n[i.0][j.0]);
        }
    }
}

#[test]
fn mat_add() {
    let m = Mat2::new(&[[-1, 4], [8, -256]]);
    let n = Mat2::new(&[[-1, 2], [10, 202]]);
    let o = &m + &n;
    for i in 0..2 {
        for j in 0..2 {
            assert_eq!(o[i][j], m[i][j] + n[i][j]);
        }
    }
}

#[test]
fn mat_add_assign() {
    let mut m = Mat2::new(&[[-1, 4], [8, -256]]);
    let n = Mat2::new(&[[-1, 2], [10, 202]]);
    let o = m.clone();
    m += &n;
    for i in 0..2 {
        for j in 0..2 {
            assert_eq!(m[i][j], o[i][j] + n[i][j]);
        }
    }
}

#[test]
fn mat_sub() {
    let m = Mat2::new(&[[-1, 4], [8, -256]]);
    let n = Mat2::new(&[[-1, 2], [10, 202]]);
    let o = &m - &n;
    for i in 0..2 {
        for j in 0..2 {
            assert_eq!(o[i][j], m[i][j] - n[i][j]);
        }
    }
}

#[test]
fn mat_sub_assign() {
    let mut m = Mat2::new(&[[-1, 4], [8, -256]]);
    let n = Mat2::new(&[[-1, 2], [10, 202]]);
    let o = m.clone();
    m -= &n;
    for i in 0..2 {
        for j in 0..2 {
            assert_eq!(m[i][j], o[i][j] - n[i][j]);
        }
    }
}

#[test]
fn mat_mul() {
    let m = Mat2::new(&[[2, 3], [4, 5]]);
    let n = Mat2::new(&[[2, 1], [1, 2]]);
    let o = &m * &n;
    assert_eq!(o[0][0], 8);
    assert_eq!(o[0][1], 11);
    assert_eq!(o[1][0], 10);
    assert_eq!(o[1][1], 13);
    let v = &m * &Vec2::new(&[10, -20]);
    assert_eq!(v[0], -60);
    assert_eq!(v[1], -70);
}

#[test]
fn mat_mul_assign() {
    let mut m = Mat2::new(&[[2, 3], [4, 5]]);
    let n = Mat2::new(&[[2, 1], [1, 2]]);
    m *= &n;
    assert_eq!(m[0][0], 8);
    assert_eq!(m[0][1], 11);
    assert_eq!(m[1][0], 10);
    assert_eq!(m[1][1], 13);
}

#[test]
fn mat_transpose() {
    let m = Mat4::new(&[
        [0.0, 1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0, 7.0],
        [8.0, 9.0, 10.0, 11.0],
        [12.0, 13.0, 14.0, 15.0],
    ]);
    let n = m.transpose();
    assert_eq!(n[0][0], m[0][0]);
    assert_eq!(n[0][1], m[1][0]);
    assert_eq!(n[0][2], m[2][0]);
    assert_eq!(n[0][3], m[3][0]);
    assert_eq!(n[1][0], m[0][1]);
    assert_eq!(n[1][1], m[1][1]);
    assert_eq!(n[1][2], m[2][1]);
    assert_eq!(n[1][3], m[3][1]);
    assert_eq!(n[2][0], m[0][2]);
    assert_eq!(n[2][1], m[1][2]);
    assert_eq!(n[2][2], m[2][2]);
    assert_eq!(n[2][3], m[3][2]);
    assert_eq!(n[3][0], m[0][3]);
    assert_eq!(n[3][1], m[1][3]);
    assert_eq!(n[3][2], m[2][3]);
    assert_eq!(n[3][3], m[3][3]);
}

#[test]
fn mat_invert() {
    let assert0 = |x: f64| assert!(x.abs() - 0.0 < 0.000000000001);
    let assert1 = |x: f64| assert!((x.abs() - 1.0).abs() < 0.000000000001);

    let m = Mat2::new(&[[12f64, 0.0], [-1.0, 4.0]]);
    let n = m.invert();
    let o = &m * &n;
    assert1(o[0][0]);
    assert0(o[0][1]);
    assert0(o[1][0]);
    assert1(o[1][1]);

    let m = Mat3::new(&[[1f64, 0.0, 0.0], [0.0, 1.0, 0.0], [7.0, 8.0, 9.0]]);
    let n = m.invert();
    let o = &m * &n;
    assert1(o[0][0]);
    assert0(o[0][1]);
    assert0(o[0][2]);
    assert0(o[1][0]);
    assert1(o[1][1]);
    assert0(o[1][2]);
    assert0(o[2][0]);
    assert0(o[2][1]);
    assert1(o[2][2]);

    let m = Mat4::new(&[
        [-2f64, 0.0, 0.0, 0.0],
        [0.0, -34.0, 0.0, 1.0],
        [0.0, 0.0, -1.0, 2.0],
        [0.0, 1.0, 2.0, -16.0],
    ]);
    let n = m.invert();
    let o = &m * &n;
    assert1(o[0][0]);
    assert0(o[0][1]);
    assert0(o[0][2]);
    assert0(o[0][3]);
    assert0(o[1][0]);
    assert1(o[1][1]);
    assert0(o[1][2]);
    assert0(o[1][3]);
    assert0(o[2][0]);
    assert0(o[2][1]);
    assert1(o[2][2]);
    assert0(o[2][3]);
    assert0(o[3][0]);
    assert0(o[3][1]);
    assert0(o[3][2]);
    assert1(o[3][3]);
}
