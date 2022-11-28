// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

use crate::linear::{Vec2, Vec3, Vec4};

/// Column-major 2x2 matrix.
#[derive(Clone, Default, Debug)]
pub struct Mat2<T>([Vec2<T>; 2]);

/// Column-major 3x3 matrix.
#[derive(Clone, Default, Debug)]
pub struct Mat3<T>([Vec3<T>; 3]);

/// Column-major 4x4 matrix.
#[derive(Clone, Default, Debug)]
pub struct Mat4<T>([Vec4<T>; 4]);

macro_rules! new_impl {
    ($m:ty, $v:ty, $n:literal) => {
        impl<T: Copy + Default> $m {
            pub fn new(m: &[[T; $n]; $n]) -> Self {
                let mut cols = [<$v>::default(); $n];
                for i in 0..$n {
                    cols[i] = <$v>::new(&m[i]);
                }
                Self(cols)
            }
        }
    };
}

new_impl!(Mat2<T>, Vec2<T>, 2);
new_impl!(Mat3<T>, Vec3<T>, 3);
new_impl!(Mat4<T>, Vec4<T>, 4);

macro_rules! index_impl {
    ($m:ty, $v:ty) => {
        impl<T> Index<usize> for $m {
            type Output = $v;

            fn index(&self, index: usize) -> &Self::Output {
                &self.0[index]
            }
        }

        impl<T> IndexMut<usize> for $m {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.0[index]
            }
        }
    };
}

index_impl!(Mat2<T>, Vec2<T>);
index_impl!(Mat3<T>, Vec3<T>);
index_impl!(Mat4<T>, Vec4<T>);

macro_rules! add_impl {
    ($m:ty, $v:ty, $n:literal) => {
        impl<T: Copy + Default + Add<Output = T>> Add for &$m {
            type Output = $m;

            fn add(self, other: Self) -> Self::Output {
                let mut m = <$m>::default();
                for i in 0..$n {
                    m[i] = &self[i] + &other[i];
                }
                m
            }
        }
    };
}

add_impl!(Mat2<T>, Vec2<T>, 2);
add_impl!(Mat3<T>, Vec3<T>, 3);
add_impl!(Mat4<T>, Vec4<T>, 4);

macro_rules! add_assign_impl {
    ($t:ty, $n:literal) => {
        impl<T: Copy + AddAssign> AddAssign<&$t> for $t {
            fn add_assign(&mut self, other: &Self) {
                for i in 0..$n {
                    self[i] += &other[i];
                }
            }
        }
    };
}

add_assign_impl!(Mat2<T>, 2);
add_assign_impl!(Mat3<T>, 3);
add_assign_impl!(Mat4<T>, 4);

macro_rules! sub_impl {
    ($m:ty, $v:ty, $n:literal) => {
        impl<T: Copy + Default + Sub<Output = T>> Sub for &$m {
            type Output = $m;

            fn sub(self, other: Self) -> Self::Output {
                let mut m = <$m>::default();
                for i in 0..$n {
                    m[i] = &self[i] - &other[i];
                }
                m
            }
        }
    };
}

sub_impl!(Mat2<T>, Vec2<T>, 2);
sub_impl!(Mat3<T>, Vec3<T>, 3);
sub_impl!(Mat4<T>, Vec4<T>, 4);

macro_rules! sub_assign_impl {
    ($t:ty, $n:literal) => {
        impl<T: Copy + SubAssign> SubAssign<&$t> for $t {
            fn sub_assign(&mut self, other: &Self) {
                for i in 0..$n {
                    self[i] -= &other[i];
                }
            }
        }
    };
}

sub_assign_impl!(Mat2<T>, 2);
sub_assign_impl!(Mat3<T>, 3);
sub_assign_impl!(Mat4<T>, 4);

macro_rules! mul_impl {
    ($m:ty, $v:ty, $n:literal) => {
        impl<T: Copy + Default + AddAssign + Mul<Output = T>> Mul for &$m {
            type Output = $m;

            fn mul(self, other: Self) -> Self::Output {
                let mut m = <$m>::default();
                for i in 0..$n {
                    for j in 0..$n {
                        for k in 0..$n {
                            m[i][j] += self[k][j] * other[i][k];
                        }
                    }
                }
                m
            }
        }

        impl<T: Copy + Default + AddAssign + Mul<Output = T>> Mul<&$v> for &$m {
            type Output = $v;

            fn mul(self, vector: &$v) -> Self::Output {
                let mut v = <$v>::default();
                for i in 0..$n {
                    for j in 0..$n {
                        v[i] += self[j][i] * vector[j];
                    }
                }
                v
            }
        }
    };
}

mul_impl!(Mat2<T>, Vec2<T>, 2);
mul_impl!(Mat3<T>, Vec3<T>, 3);
mul_impl!(Mat4<T>, Vec4<T>, 4);

macro_rules! mul_assign_impl {
    ($t:ty) => {
        impl<T: Copy + Default + AddAssign + Mul<Output = T>> MulAssign<&$t> for $t {
            fn mul_assign(&mut self, other: &Self) {
                *self = &*self * other;
                //let m = self.clone();
                //*self = &m * other;
            }
        }
    };
}

mul_assign_impl!(Mat2<T>);
mul_assign_impl!(Mat3<T>);
mul_assign_impl!(Mat4<T>);

macro_rules! transpose_impl {
    ($t:ty, $n:literal) => {
        impl<T: Copy + Default> $t {
            pub fn transpose(&self) -> Self {
                let mut m = <$t>::default();
                for i in 0..$n {
                    m[i][i] = self[i][i];
                    for j in i + 1..$n {
                        m[i][j] = self[j][i];
                        m[j][i] = self[i][j];
                    }
                }
                m
            }
        }
    };
}

transpose_impl!(Mat2<T>, 2);
transpose_impl!(Mat3<T>, 3);
transpose_impl!(Mat4<T>, 4);

// NOTE: Floating-point only.
macro_rules! invert_impl {
    ($($f:ty, $one:literal),+) => {$(
        impl Mat2<$f> {
            pub fn invert(&self) -> Self {
                let m00 = self[0][0];
                let m01 = self[0][1];
                let m10 = self[1][0];
                let m11 = self[1][1];
                let det = m00 * m11 - m01 * m10;
                let idet = $one / det;
                Self::new(&[[m11 * idet, m01 * idet], [-m10 * idet, m00 * idet]])
            }
        }

        impl Mat3<$f> {
            pub fn invert(&self) -> Self {
                let m00 = self[0][0];
                let m01 = self[0][1];
                let m02 = self[0][2];
                let m10 = self[1][0];
                let m11 = self[1][1];
                let m12 = self[1][2];
                let m20 = self[2][0];
                let m21 = self[2][1];
                let m22 = self[2][2];
                let s0 = m11 * m22 - m12 * m21;
                let s1 = m10 * m22 - m12 * m20;
                let s2 = m10 * m21 - m11 * m20;
                let det = m00 * s0 - m01 * s1 + m02 * s2;
                let idet = $one / det;
                Self::new(&[
                    [
                        s0 * idet,
                        -(m01 * m22 - m02 * m21) * idet,
                        (m01 * m12 - m02 * m11) * idet,
                    ],
                    [
                        -s1 * idet,
                        (m00 * m22 - m02 * m20) * idet,
                        -(m00 * m12 - m02 * m10) * idet,
                    ],
                    [
                        s2 * idet,
                        -(m00 * m21 - m01 * m20) * idet,
                        (m00 * m11 - m01 * m10) * idet,
                    ],
                ])
            }
        }

        impl Mat4<$f> {
            pub fn invert(&self) -> Self {
                let m00 = self[0][0];
                let m01 = self[0][1];
                let m02 = self[0][2];
                let m03 = self[0][3];
                let m10 = self[1][0];
                let m11 = self[1][1];
                let m12 = self[1][2];
                let m13 = self[1][3];
                let m20 = self[2][0];
                let m21 = self[2][1];
                let m22 = self[2][2];
                let m23 = self[2][3];
                let m30 = self[3][0];
                let m31 = self[3][1];
                let m32 = self[3][2];
                let m33 = self[3][3];
                let s0 = m00 * m11 - m01 * m10;
                let s1 = m00 * m12 - m02 * m10;
                let s2 = m00 * m13 - m03 * m10;
                let s3 = m01 * m12 - m02 * m11;
                let s4 = m01 * m13 - m03 * m11;
                let s5 = m02 * m13 - m03 * m12;
                let c0 = m20 * m31 - m21 * m30;
                let c1 = m20 * m32 - m22 * m30;
                let c2 = m20 * m33 - m23 * m30;
                let c3 = m21 * m32 - m22 * m31;
                let c4 = m21 * m33 - m23 * m31;
                let c5 = m22 * m33 - m23 * m32;
                let det = s0 * c5 - s1 * c4 + s2 * c3 + s3 * c2 - s4 * c1 + s5 * c0;
                let idet = $one / det;
                Self::new(&[
                    [
                        (c5 * m11 - c4 * m12 + c3 * m13) * idet,
                        (-c5 * m01 + c4 * m02 - c3 * m03) * idet,
                        (s5 * m31 - s4 * m32 + s3 * m33) * idet,
                        (-s5 * m21 + s4 * m22 - s3 * m23) * idet,
                    ],
                    [
                        (-c5 * m10 + c2 * m12 - c1 * m13) * idet,
                        (c5 * m00 - c2 * m02 + c1 * m03) * idet,
                        (-s5 * m30 + s2 * m32 - s1 * m33) * idet,
                        (s5 * m20 - s2 * m22 + s1 * m23) * idet,
                    ],
                    [
                        (c4 * m10 - c2 * m11 + c0 * m13) * idet,
                        (-c4 * m00 + c2 * m01 - c0 * m03) * idet,
                        (s4 * m30 - s2 * m31 + s0 * m33) * idet,
                        (-s4 * m20 + s2 * m21 - s0 * m23) * idet,
                    ],
                    [
                        (-c3 * m10 + c1 * m11 - c0 * m12) * idet,
                        (c3 * m00 - c1 * m01 + c0 * m02) * idet,
                        (-s3 * m30 + s1 * m31 - s0 * m32) * idet,
                        (s3 * m20 - s1 * m21 + s0 * m22) * idet,
                    ],
                ])
            }
        }
    )+};
}

invert_impl!(f32, 1f32, f64, 1f64);
