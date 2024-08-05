use crate::linear::{Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec4};

#[test]
fn vec_index() {
    let a = [-1, 0, 2047];
    let v = Vec3::from(a);
    for i in a.iter().enumerate() {
        assert_eq!(*i.1, v[i.0]);
    }
    let mut v = Vec3::from([0; 3]);
    v[1] = a[1];
    v[0] = a[2];
    v[2] = a[0];
    for i in a.iter().rev().enumerate() {
        assert_eq!(*i.1, v[i.0]);
    }
}

#[test]
fn vec_add() {
    let v = Vec4::from([1i8; 4]);
    let u = Vec4::new(-2i8, -3, 0, 1);
    let w = &v + &u;
    let x = u + v;
    for i in 0..4 {
        assert_eq!(w[i], v[i] + u[i]);
        assert_eq!(w[i], x[i]);
    }
}

#[test]
fn vec_add_assign() {
    let mut v = Vec2::new(4, 2);
    let u = Vec2::new(-10, 10);
    let mut w = v;
    v += &u;
    for i in 0..2 {
        assert_eq!(v[i], w[i] + u[i]);
    }
    w += u;
    for i in 0..2 {
        assert_eq!(w[i], v[i]);
    }
}

#[test]
fn vec_sub() {
    let v = Vec4::from([1i8; 4]);
    let u = Vec4::new(-2i8, 3, 0, -1);
    let w = v - u;
    let x = &u - &v;
    for i in 0..4 {
        assert_eq!(w[i], v[i] - u[i]);
        assert_ne!(w[i], x[i]);
        assert_eq!(x[i], u[i] - v[i]);
    }
}

#[test]
fn vec_sub_assign() {
    let mut v = Vec2::new(4, 2);
    let u = Vec2::new(-10, 10);
    let mut w = v;
    v -= u;
    for i in 0..2 {
        assert_eq!(v[i], w[i] - u[i]);
    }
    w -= &u;
    for i in 0..2 {
        assert_eq!(w[i], v[i]);
    }
}

#[test]
fn vec_mul() {
    let a = [1u64, 99, 65535];
    let v = Vec3::from(a);
    let s = 4096;
    let v = &v * s;
    for i in 0..3 {
        assert_eq!(v[i], a[i] * s);
    }
    let v = v * 0;
    for i in 0..3 {
        assert_eq!(v[i], 0);
    }
}

#[test]
fn vec_mul_assign() {
    let mut v = Vec3::new(-1i64, -255, 256);
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
    let v = Vec3::from(a);
    let s = 9;
    let v = v / s;
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
    let mut v = Vec3::new(-1i64, -255, 256);
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
    let v = Vec4::new(-0.5, -1.0, 0.0, 0.125);
    let u = -&v;
    for i in 0..4 {
        assert_eq!(v[i], -u[i]);
    }
    let u = -u;
    for i in 0..4 {
        assert_eq!(v[i], u[i]);
    }
}

#[test]
fn vec_dot() {
    let v = Vec3::new(0f32, 0.7071068, 0.7071068);
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
    let v = Vec2::new(4f32, 3f32);
    let s = v.length();
    assert_eq!(s, 5.0);
    let v = Vec4::new(4f32 / s, 3f32 / s, 0.0, 0.0);
    let s = v.length();
    assert_eq!(s, 1.0);
}

#[test]
fn vec_normalize() {
    let v = Vec3::new(3f64, 0.0, 4f64);
    let u = v.normalize();
    assert_eq!(u[0], 0.6);
    assert_eq!(u[1], 0.0);
    assert_eq!(u[2], 0.8);
    assert_eq!(u.length(), 1.0);
}

#[test]
fn vec_cross() {
    let v = Vec3::<f32>::new(0.0, 0.0, 1.0);
    let u = Vec3::<f32>::new(0.0, 1.0, 0.0);
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
    let m = Mat3::new(a[0], a[1], a[2]);
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
    let m = Mat2::new([-1, 4], [8, -256]);
    let n = Mat2::new([-1, 2], [10, 202]);
    let o = &m + &n;
    for i in 0..2 {
        for j in 0..2 {
            assert_eq!(o[i][j], m[i][j] + n[i][j]);
        }
    }
    let p = m + n;
    for i in 0..2 {
        for j in 0..2 {
            assert_eq!(p[i][j], o[i][j]);
        }
    }
}

#[test]
fn mat_add_assign() {
    let mut m = Mat2::new([-1, 4], [8, -256]);
    let n = Mat2::new([-1, 2], [10, 202]);
    let mut o = m;
    m += &n;
    for i in 0..2 {
        for j in 0..2 {
            assert_eq!(m[i][j], o[i][j] + n[i][j]);
        }
    }
    o += n;
    for i in 0..2 {
        for j in 0..2 {
            assert_eq!(o[i][j], m[i][j]);
        }
    }
}

#[test]
fn mat_sub() {
    let m = Mat2::new([-1, 4], [8, -256]);
    let n = Mat2::new([-1, 2], [10, 202]);
    let o = &m - &n;
    for i in 0..2 {
        for j in 0..2 {
            assert_eq!(o[i][j], m[i][j] - n[i][j]);
        }
    }
    let p = m - n;
    for i in 0..2 {
        for j in 0..2 {
            assert_eq!(p[i][j], o[i][j]);
        }
    }
}

#[test]
fn mat_sub_assign() {
    let mut m = Mat2::new([-1, 4], [8, -256]);
    let n = Mat2::new([-1, 2], [10, 202]);
    let mut o = m;
    m -= &n;
    for i in 0..2 {
        for j in 0..2 {
            assert_eq!(m[i][j], o[i][j] - n[i][j]);
        }
    }
    o -= n;
    for i in 0..2 {
        for j in 0..2 {
            assert_eq!(o[i][j], m[i][j]);
        }
    }
}

#[test]
fn mat_mul() {
    let m = Mat2::new([2, 3], [4, 5]);
    let n = Mat2::new([2, 1], [1, 2]);
    let o = &m * &n;
    assert_eq!(o[0][0], 8);
    assert_eq!(o[0][1], 11);
    assert_eq!(o[1][0], 10);
    assert_eq!(o[1][1], 13);
    let v = &m * &Vec2::new(10, -20);
    assert_eq!(v[0], -60);
    assert_eq!(v[1], -70);
    let u = &m * Vec2::new(10, -20);
    assert_eq!(u[0], v[0]);
    assert_eq!(u[1], v[1]);
    let u = m * &Vec2::new(10, -20);
    assert_eq!(u[0], v[0]);
    assert_eq!(u[1], v[1]);
    let u = m * Vec2::new(10, -20);
    assert_eq!(u[0], v[0]);
    assert_eq!(u[1], v[1]);
    let p = m * n;
    assert_eq!(p[0][0], o[0][0]);
    assert_eq!(p[0][1], o[0][1]);
    assert_eq!(p[1][0], o[1][0]);
    assert_eq!(p[1][1], o[1][1]);
}

#[test]
fn mat_mul_assign() {
    let mut m = Mat2::new([2, 3], [4, 5]);
    let n = Mat2::new([2, 1], [1, 2]);
    let mut o = m;
    m *= &n;
    assert_eq!(m[0][0], 8);
    assert_eq!(m[0][1], 11);
    assert_eq!(m[1][0], 10);
    assert_eq!(m[1][1], 13);
    o *= n;
    assert_eq!(o[0][0], m[0][0]);
    assert_eq!(o[0][1], m[0][1]);
    assert_eq!(o[1][0], m[1][0]);
    assert_eq!(o[1][1], m[1][1]);
}

#[test]
fn mat_neg() {
    let m = Mat3::new([-1.0, 2.0, 0.25], [-5.0, 0.1, 5.0], [999.9, -0.0, 0.001]);
    let n = -&m;
    let o = -n;
    for i in 0..3 {
        for j in 0..3 {
            assert_eq!(n[i][j], -m[i][j]);
            assert_eq!(o[i][j], -n[i][j]);
        }
    }
}

#[test]
fn mat_transpose() {
    let m = Mat4::new(
        [0.0, 1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0, 7.0],
        [8.0, 9.0, 10.0, 11.0],
        [12.0, 13.0, 14.0, 15.0],
    );
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
fn mat_det() {
    let m = Mat2::new([1.0, -1.0], [2.0, -0.5]);
    assert_eq!(m.det(), m[0][0] * m[1][1] - m[0][1] * m[1][0]);
    let m = Mat3::from([1; 9]);
    assert_eq!(m.det(), 0);
    let mut m = <Mat4<f32>>::default();
    m[0][0] = 0.5;
    m[1][1] = -2.0;
    m[2][2] = 8.0;
    m[3][3] = 1.5;
    assert_eq!(m.det(), m[0][0] * m[1][1] * m[2][2] * m[3][3]);
}

#[test]
fn mat_invert() {
    let assert0 = |x: f64| assert!(x.abs() <= f64::EPSILON);
    let assert1 = |x: f64| assert!((x.abs() - 1.0).abs() <= f64::EPSILON);

    let m = Mat2::new([12f64, 0.0], [-1.0, 4.0]);
    let n = m.invert();
    let o = &m * &n;
    assert1(o[0][0]);
    assert0(o[0][1]);
    assert0(o[1][0]);
    assert1(o[1][1]);

    let m = Mat3::new([1f64, 0.0, 0.0], [0.0, 1.0, 0.0], [7.0, 8.0, 9.0]);
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

    let m = Mat4::new(
        [-2f64, 0.0, 0.0, 0.0],
        [0.0, -34.0, 0.0, 1.0],
        [0.0, 0.0, -1.0, 2.0],
        [0.0, 1.0, 2.0, -16.0],
    );
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

#[test]
fn quat_mul() {
    let q = Quat::new([0.0; 3], 1f32);
    let u = Quat::new([0.7071068, 0.0, -0.7071068], 1f32);
    let p = &q * &u;
    assert_eq!(p.imag()[0], u.imag()[0]);
    assert_eq!(p.imag()[1], u.imag()[1]);
    assert_eq!(p.imag()[2], u.imag()[2]);
    assert_eq!(p.real(), u.real());
    let p = u * q;
    assert_eq!(p.imag()[0], u.imag()[0]);
    assert_eq!(p.imag()[1], u.imag()[1]);
    assert_eq!(p.imag()[2], u.imag()[2]);
    assert_eq!(p.real(), u.real());
}

#[test]
fn quat_mul_assign() {
    const PI: f64 = std::f64::consts::PI;
    let mut q = Quat::new([0.0, 0.0, -1.0], PI);
    let u = Quat::new([1.0, 0.0, 0.0], PI);
    let mut p = q;
    q *= &u;
    assert!((q.imag()[0] - PI).abs() <= f64::EPSILON);
    assert_eq!(q.imag()[1], -1.0);
    assert!((q.imag()[2] + PI).abs() <= f64::EPSILON);
    assert!((q.real() - PI * PI).abs() <= f64::EPSILON);
    p *= u;
    assert_eq!(p.imag()[0], q.imag()[0]);
    assert_eq!(p.imag()[1], q.imag()[1]);
    assert_eq!(p.imag()[2], q.imag()[2]);
    assert_eq!(p.real(), q.real());
}

#[test]
fn quat_rotation() {
    const PI_2: f64 = std::f64::consts::FRAC_PI_2;
    const PI_4: f64 = PI_2 / 2.0;

    let q = Quat::rotation(PI_2, &Vec3::new(1.0, 0.0, 0.0));
    let u = Quat::rotation(PI_2, &Vec3::new(0.0, 1.0, 0.0));
    let p = &q * &u;
    assert!((p.imag()[0] - 0.5).abs() <= f64::EPSILON);
    assert!((p.imag()[1] - 0.5).abs() <= f64::EPSILON);
    assert!((p.imag()[2] - 0.5).abs() <= f64::EPSILON);
    assert!((p.real() - 0.5).abs() <= f64::EPSILON);

    let r = Quat::rotation_m(&Mat3::rotation_q(&q));
    assert!((r.imag()[0] - q.imag()[0]).abs() <= f64::EPSILON);
    assert!((r.imag()[1] - q.imag()[1]).abs() <= f64::EPSILON);
    assert!((r.imag()[2] - q.imag()[2]).abs() <= f64::EPSILON);
    assert!((r.real() - q.real()).abs() <= f64::EPSILON);
    let r = Quat::rotation_m(&Mat3::rotation_q(&u));
    assert!((r.imag()[0] - u.imag()[0]).abs() <= f64::EPSILON);
    assert!((r.imag()[1] - u.imag()[1]).abs() <= f64::EPSILON);
    assert!((r.imag()[2] - u.imag()[2]).abs() <= f64::EPSILON);
    assert!((r.real() - u.real()).abs() <= f64::EPSILON);
    let r = Quat::rotation_m(&Mat3::rotation_q(&p));
    assert!((r.imag()[0] - p.imag()[0]).abs() <= f64::EPSILON);
    assert!((r.imag()[1] - p.imag()[1]).abs() <= f64::EPSILON);
    assert!((r.imag()[2] - p.imag()[2]).abs() <= f64::EPSILON);
    assert!((r.real() - p.real()).abs() <= f64::EPSILON);

    let q = Quat::rotation_x(PI_4);
    let u = Quat::rotation_y(PI_4);
    let p = Quat::rotation_z(PI_4);
    assert_eq!(q.imag()[0], u.imag()[1]);
    assert_eq!(u.imag()[1], p.imag()[2]);
    assert_eq!(q.imag()[1], 0.0);
    assert_eq!(q.imag()[2], 0.0);
    assert_eq!(u.imag()[0], 0.0);
    assert_eq!(u.imag()[2], 0.0);
    assert_eq!(p.imag()[0], 0.0);
    assert_eq!(p.imag()[1], 0.0);
    assert_eq!(u.real(), q.real());
    assert_eq!(q.real(), p.real());
}

#[test]
fn mat_translation() {
    let m = Mat4::translation(3, -2, -99);
    for i in 0..3 {
        assert_eq!(m[i][i], 1);
        for j in i + 1..3 {
            assert_eq!(m[i][j], 0);
            assert_eq!(m[j][i], 0);
        }
    }
    assert_eq!(m[3][0], 3);
    assert_eq!(m[3][1], -2);
    assert_eq!(m[3][2], -99);
    assert_eq!(m[3][3], 1);
}

#[test]
fn mat_rotation() {
    const PI: f32 = std::f32::consts::PI;
    const PI_2: f32 = PI / 2.0;
    const PI_4: f32 = PI_2 / 2.0;
    let assert = |m: Mat4<f32>, n: Mat4<f32>| {
        for i in 0..4 {
            for j in 0..4 {
                assert!((m[i][j] - n[i][j]).abs() <= f32::EPSILON);
            }
        }
    };

    let m = Mat4::rotation(PI, &Vec3::new(1.0, 0.0, 0.0));
    let n = Mat4::rotation_x(PI);
    assert(m, n);

    let m = Mat4::rotation(PI_2, &Vec3::new(0.0, 1.0, 0.0));
    let n = Mat4::rotation_y(PI_2);
    assert(m, n);

    let m = Mat4::rotation(PI_4, &Vec3::new(0.0, 0.0, -1.0));
    let n = Mat4::rotation_z(-PI_4);
    assert(m, n);

    let q = Quat::rotation(PI_2, &Vec3::new(0.0, -1.0, 0.0));
    let m = Mat4::rotation_q(&q);
    let n = Mat4::rotation(PI_2, &Vec3::new(0.0, -1.0, 0.0));
    assert(m, n);
}

#[test]
fn mat_scale() {
    let a = [2.0, 3.0, 0.5, 1.0];
    let m = Mat3::scale(a[0], a[1], a[2]);
    for i in 0..3 {
        assert_eq!(m[i][i], a[i]);
        for j in i + 1..3 {
            assert_eq!(m[i][j], 0.0);
            assert_eq!(m[j][i], 0.0);
        }
    }
    let m = Mat4::scale(a[0], a[1], a[2]);
    for i in 0..4 {
        assert_eq!(m[i][i], a[i]);
        for j in i + 1..4 {
            assert_eq!(m[i][j], 0.0);
            assert_eq!(m[j][i], 0.0);
        }
    }
}

#[test]
fn mat_view() {
    let center = Vec3::default();
    let eye = Vec3::new(-1.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let m = Mat4::look_at(&center, &eye, &up);
    assert_eq!(m[0][2], -1.0);
    assert_eq!(m[1][1], -1.0);
    assert_eq!(m[2][0], 1.0);
    assert_eq!(m[3][2], -1.0);
}

#[test]
fn mat_projection() {
    let yfov = std::f64::consts::FRAC_PI_2;
    let aspect = 16.0 / 9.0;
    let (znear, zfar) = (0.01, 100.0);

    let m = Mat4::perspective(yfov, aspect, znear, zfar);
    assert!(((m[0][0] * aspect) - 1.0).abs() <= f64::EPSILON);
    assert_eq!(m[3][3], 0.0);
    let n = Mat4::inf_perspective(yfov, aspect, znear);
    assert!(((n[0][0] * aspect) - 1.0).abs() <= f64::EPSILON);
    assert_eq!(n[3][3], 0.0);
    assert!(n[2][2] > m[2][2]);
    assert!(n[3][2] > m[3][2]);

    let (znear, zfar) = (0.0, -1.0);
    let (xmag, ymag) = (1.25, 1.5);

    let m = Mat4::ortho(xmag, ymag, znear, zfar);
    assert_eq!(m[0][0], 1.0 / xmag);
    assert_eq!(m[1][1], 1.0 / ymag);
    assert_eq!(m[3][3], 1.0);
}

#[test]
#[allow(path_statements)]
fn vec_conv() {
    let v = Vec3::from(1.0);
    for i in 0..3 {
        assert_eq!(v[i], 1.0);
    }

    let a = [-1, -2, -3, -4];
    let v = Vec4::from(a);
    let u = Vec4::from(&a);
    for i in 0..4 {
        assert_eq!(v[i], a[i]);
        assert_eq!(u[i], v[i]);
    }

    let a: [i32; 4] = v.into();
    let b: [i32; 4] = (&u).into();
    for i in 0..4 {
        assert_eq!(a[i], v[i]);
        assert_eq!(b[i], v[i]);
    }

    let m = Mat4::scale(2f32, 3.0, 4.0);
    let v = Vec4::from(m);
    for i in 0..4 {
        assert_eq!(v[i], m[i][i]);
    }

    let u = Vec3::from(&v);
    let w = Vec3::from(v);
    for i in 0..3 {
        assert_eq!(u[i], v[i]);
        assert_eq!(w[i], v[i]);
    }

    let q = Quat::new([-0.7071068, 0.7071068, 0.0], 1.0);
    let v = Vec4::from(q);
    for i in 0..3 {
        assert_eq!(v[i], q.imag()[i]);
    }
    assert_eq!(v[3], q.real());
    q;
}

#[test]
fn mat_conv() {
    let m = Mat4::from(1f32);
    for i in 0..4 {
        assert_eq!(m[i][i], 1f32);
        for j in i + 1..4 {
            assert_eq!(m[i][j], 0f32);
            assert_eq!(m[i][j], 0f32);
        }
    }

    let v = Vec3::new(1, 2, 3);
    let m = [Mat3::from(&v), Mat3::from(v)];
    for i in 0..3 {
        assert_eq!(m[0][i][i], v[i]);
        assert_eq!(m[1][i][i], v[i]);
        for j in i + 1..3 {
            assert_eq!(m[0][i][j], 0);
            assert_eq!(m[0][i][j], 0);
            assert_eq!(m[1][i][j], 0);
            assert_eq!(m[1][i][j], 0);
        }
    }

    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let m = Mat3::from(a);
    let n = Mat3::from(&a);
    for i in 0..3 {
        for j in i + 1..3 {
            assert_eq!(m[i][j], a[i * 3 + j]);
            assert_eq!(n[i][j], m[i][j]);
        }
    }

    let m = Mat3::new([1, 2, 3], [4, 5, 6], [7, 8, 9]);
    let n = Mat4::from(&m);
    for i in 0..3 {
        for j in 0..3 {
            assert_eq!(m[i][j], n[i][j]);
        }
    }
    assert_eq!(n[3][0], 0);
    assert_eq!(n[3][1], 0);
    assert_eq!(n[3][2], 0);
    assert_eq!(n[3][3], 1);

    let n = &n + &n;
    let m = Mat3::from(n);
    for i in 0..3 {
        for j in 0..3 {
            assert_eq!(m[i][j], n[i][j]);
        }
    }
}

#[test]
fn quat_conv() {
    let v = Vec4::new(0.7071068, 0.0, -0.7071068, 1.0);
    let q = Quat::from(v);
    assert_eq!(v[0], q.imag()[0]);
    assert_eq!(v[1], q.imag()[1]);
    assert_eq!(v[2], q.imag()[2]);
    assert_eq!(v[3], q.real());
}

#[test]
fn mat_trs() {
    let assert = |m: &Mat4<f64>, t: Vec3<f64>, r: Quat<f64>, s: Vec3<f64>| {
        let n = Mat4::translation(t[0], t[1], t[2])
            * Mat4::rotation_q(&r)
            * Mat4::scale(s[0], s[1], s[2]);
        for i in 0..4 {
            for j in 0..4 {
                assert!((m[i][j] - n[i][j]).abs() <= f64::EPSILON);
            }
        }
    };

    let t = Vec3::new(-10.0, 20.0, -30.0);
    let r = Quat::rotation_y(std::f64::consts::FRAC_PI_2);
    let s = Vec3::from(2.0);
    let m = Mat4::from_trs(&t, &r, &s);
    let trs = m.into_trs();
    assert(&m, t, r, s);
    assert(&m, trs.0, trs.1, trs.2);

    let t = Vec3::new(50.0, 0.0, -100.0);
    let r = Quat::rotation(
        std::f64::consts::FRAC_PI_3,
        &Vec3::new(0.7071068, 0.0, -0.7071068),
    );
    let s = Vec3::from(0.5);
    let m = Mat4::from_trs(&t, &r, &s);
    let trs = m.into_trs();
    assert(&m, t, r, s);
    assert(&m, trs.0, trs.1, trs.2);

    let t = Vec3::default();
    let r = Quat::rotation(
        std::f64::consts::FRAC_PI_3,
        &Vec3::new(0.7071068, 0.0, -0.7071068),
    );
    let s = Vec3::from(-1.0);
    let m = Mat4::from_trs(&t, &r, &s);
    let trs = m.into_trs();
    assert(&m, t, r, s);
    assert(&m, trs.0, trs.1, trs.2);

    let t = Vec3::new(64.0, 16.0, 4.0);
    let r = Quat::rotation_x(std::f64::consts::FRAC_PI_3)
        * Quat::rotation_y(std::f64::consts::FRAC_PI_6)
        * Quat::rotation_z(std::f64::consts::FRAC_PI_4);
    let s = Vec3::new(1.0, 2.75, 0.25);
    let m = Mat4::from_trs(&t, &r, &s);
    let trs = m.into_trs();
    assert(&m, t, r, s);
    assert(&m, trs.0, trs.1, trs.2);
}
