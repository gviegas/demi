use std::mem;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::linear::{Float, Quat, Scalar, Vec2, Vec3, Vec4};

/// Column-major 2x2 matrix.
#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub struct Mat2<T>([Vec2<T>; 2]);

/// Column-major 3x3 matrix.
#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub struct Mat3<T>([Vec3<T>; 3]);

/// Column-major 4x4 matrix.
#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub struct Mat4<T>([Vec4<T>; 4]);

impl<T> Mat2<T> {
    /// Creates a new `Mat2`.
    pub fn new(col0: [T; 2], col1: [T; 2]) -> Self {
        Self([Vec2::from(col0), Vec2::from(col1)])
    }
}

impl<T> Mat3<T> {
    /// Creates a new `Mat3`.
    pub fn new(col0: [T; 3], col1: [T; 3], col2: [T; 3]) -> Self {
        Self([Vec3::from(col0), Vec3::from(col1), Vec3::from(col2)])
    }
}

impl<T> Mat4<T> {
    /// Creates a new `Mat4`.
    pub fn new(col0: [T; 4], col1: [T; 4], col2: [T; 4], col3: [T; 4]) -> Self {
        Self([
            Vec4::from(col0),
            Vec4::from(col1),
            Vec4::from(col2),
            Vec4::from(col3),
        ])
    }
}

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

        impl<T: Copy + Default + Add<Output = T>> Add for $m {
            type Output = Self;

            fn add(self, other: Self) -> Self::Output {
                let mut m = Self::default();
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

        impl<T: Copy + AddAssign> AddAssign for $t {
            fn add_assign(&mut self, other: Self) {
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

        impl<T: Copy + Default + Sub<Output = T>> Sub for $m {
            type Output = Self;

            fn sub(self, other: Self) -> Self::Output {
                let mut m = Self::default();
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

        impl<T: Copy + SubAssign> SubAssign for $t {
            fn sub_assign(&mut self, other: Self) {
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

        impl<T: Copy + Default + AddAssign + Mul<Output = T>> Mul for $m {
            type Output = Self;

            fn mul(self, other: Self) -> Self::Output {
                let mut m = Self::default();
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

        impl<T: Copy + Default + AddAssign + Mul<Output = T>> Mul<$v> for &$m {
            type Output = $v;

            fn mul(self, vector: $v) -> Self::Output {
                let mut v = <$v>::default();
                for i in 0..$n {
                    for j in 0..$n {
                        v[i] += self[j][i] * vector[j];
                    }
                }
                v
            }
        }

        impl<T: Copy + Default + AddAssign + Mul<Output = T>> Mul<&$v> for $m {
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

        impl<T: Copy + Default + AddAssign + Mul<Output = T>> Mul<$v> for $m {
            type Output = $v;

            fn mul(self, vector: $v) -> Self::Output {
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
                //let m = *self;
                //*self = &m * other;
            }
        }

        impl<T: Copy + Default + AddAssign + Mul<Output = T>> MulAssign for $t {
            fn mul_assign(&mut self, other: Self) {
                *self = &*self * &other;
            }
        }
    };
}

mul_assign_impl!(Mat2<T>);
mul_assign_impl!(Mat3<T>);
mul_assign_impl!(Mat4<T>);

macro_rules! neg_impl {
    ($t:ty) => {
        impl<T: Copy + Neg<Output = T>> Neg for &$t {
            type Output = $t;

            fn neg(self) -> Self::Output {
                let mut m = *self;
                for i in &mut m.0 {
                    *i = -*i;
                }
                m
            }
        }

        impl<T: Copy + Neg<Output = T>> Neg for $t {
            type Output = Self;

            fn neg(self) -> Self::Output {
                let mut m = self;
                for i in &mut m.0 {
                    *i = -*i;
                }
                m
            }
        }
    };
}

neg_impl!(Mat2<T>);
neg_impl!(Mat3<T>);
neg_impl!(Mat4<T>);

macro_rules! transpose_impl {
    ($t:ty, $n:literal) => {
        impl<T: Copy + Default> $t {
            /// Computes the transpose.
            #[must_use]
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

impl<T: Scalar> Mat2<T> {
    /// Computes the determinant.
    pub fn det(&self) -> T {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }
}

impl<T: Scalar> Mat3<T> {
    /// Computes the determinant.
    pub fn det(&self) -> T {
        let m00 = self[0][0];
        let m01 = self[0][1];
        let m02 = self[0][2];
        let m10 = self[1][0];
        let m11 = self[1][1];
        let m12 = self[1][2];
        let m20 = self[2][0];
        let m21 = self[2][1];
        let m22 = self[2][2];
        m00 * (m11 * m22 - m12 * m21) - m01 * (m10 * m22 - m12 * m20)
            + m02 * (m10 * m21 - m11 * m20)
    }
}

impl<T: Scalar> Mat4<T> {
    /// Computes the determinant.
    pub fn det(&self) -> T {
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
        (m00 * m11 - m01 * m10) * (m22 * m33 - m23 * m32)
            - (m00 * m12 - m02 * m10) * (m21 * m33 - m23 * m31)
            + (m00 * m13 - m03 * m10) * (m21 * m32 - m22 * m31)
            + (m01 * m12 - m02 * m11) * (m20 * m33 - m23 * m30)
            - (m01 * m13 - m03 * m11) * (m20 * m32 - m22 * m30)
            + (m02 * m13 - m03 * m12) * (m20 * m31 - m21 * m30)
    }
}

impl<T: Float> Mat2<T> {
    /// Computes the inverse.
    ///
    /// NOTE: One must ensure that `self` is invertible.
    #[must_use]
    pub fn invert(&self) -> Self {
        let m00 = self[0][0];
        let m01 = self[0][1];
        let m10 = self[1][0];
        let m11 = self[1][1];
        let det = m00 * m11 - m01 * m10;
        let idet = T::ONE / det;
        Self::new([m11 * idet, m01 * idet], [-m10 * idet, m00 * idet])
    }
}

impl<T: Float> Mat3<T> {
    /// Computes the inverse.
    ///
    /// NOTE: One must ensure that `self` is invertible.
    #[must_use]
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
        let idet = T::ONE / det;
        Self::new(
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
        )
    }
}

impl<T: Float> Mat4<T> {
    /// Computes the inverse.
    ///
    /// NOTE: One must ensure that `self` is invertible.
    #[must_use]
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
        let idet = T::ONE / det;
        Self::new(
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
        )
    }
}

impl<T: Scalar> Mat4<T> {
    /// Creates a new matrix encoding a translation.
    pub fn translation(x: T, y: T, z: T) -> Self {
        Self::new(
            [T::ONE, T::ZERO, T::ZERO, T::ZERO],
            [T::ZERO, T::ONE, T::ZERO, T::ZERO],
            [T::ZERO, T::ZERO, T::ONE, T::ZERO],
            [x, y, z, T::ONE],
        )
    }
}

impl<T: Float> Mat3<T> {
    /// Creates a new matrix encoding a rotation about an arbitrary axis.
    pub fn rotation(angle: T, axis: &Vec3<T>) -> Self {
        let axis = axis.normalize();
        let (x, y, z) = (axis[0], axis[1], axis[2]);
        let cos = angle.cos();
        let sin = angle.sin();
        let dcos = T::ONE - cos;
        let dcosxy = dcos * x * y;
        let dcosxz = dcos * x * z;
        let dcosyz = dcos * y * z;
        let sinx = sin * x;
        let siny = sin * y;
        let sinz = sin * z;
        Self::new(
            [cos + dcos * x * x, dcosxy + sinz, dcosxz - siny],
            [dcosxy - sinz, cos + dcos * y * y, dcosyz + sinx],
            [dcosxz + siny, dcosyz - sinx, cos + dcos * z * z],
        )
    }

    /// Creates a new matrix encoding the rotation described by a given quaternion.
    pub fn rotation_q(quat: &Quat<T>) -> Self {
        // TODO: Implement vector conversions.
        let imag = quat.imag();
        let real = quat.real();
        let qvec = Vec4::new(imag[0], imag[1], imag[2], real).normalize();
        let (x, y, z, w) = (qvec[0], qvec[1], qvec[2], qvec[3]);
        let xx2 = (T::ONE + T::ONE) * x * x;
        let xy2 = (T::ONE + T::ONE) * x * y;
        let xz2 = (T::ONE + T::ONE) * x * z;
        let xw2 = (T::ONE + T::ONE) * x * w;
        let yy2 = (T::ONE + T::ONE) * y * y;
        let yz2 = (T::ONE + T::ONE) * y * z;
        let yw2 = (T::ONE + T::ONE) * y * w;
        let zz2 = (T::ONE + T::ONE) * z * z;
        let zw2 = (T::ONE + T::ONE) * z * w;
        Self::new(
            [T::ONE - yy2 - zz2, xy2 + zw2, xz2 - yw2],
            [xy2 - zw2, T::ONE - xx2 - zz2, yz2 + xw2],
            [xz2 + yw2, yz2 - xw2, T::ONE - xx2 - yy2],
        )
    }

    /// Creates a new matrix encoding a rotation about the x axis.
    pub fn rotation_x(angle: T) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self::new(
            [T::ONE, T::ZERO, T::ZERO],
            [T::ZERO, cos, sin],
            [T::ZERO, -sin, cos],
        )
    }

    /// Creates a new matrix encoding a rotation about the y axis.
    pub fn rotation_y(angle: T) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self::new(
            [cos, T::ZERO, -sin],
            [T::ZERO, T::ONE, T::ZERO],
            [sin, T::ZERO, cos],
        )
    }

    /// Creates a new matrix encoding a rotation about the z axis.
    pub fn rotation_z(angle: T) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self::new(
            [cos, sin, T::ZERO],
            [-sin, cos, T::ZERO],
            [T::ZERO, T::ZERO, T::ONE],
        )
    }
}

impl<T: Float> Mat4<T> {
    /// Creates a new matrix encoding a rotation about an arbitrary axis.
    pub fn rotation(angle: T, axis: &Vec3<T>) -> Self {
        let axis = axis.normalize();
        let (x, y, z) = (axis[0], axis[1], axis[2]);
        let cos = angle.cos();
        let sin = angle.sin();
        let dcos = T::ONE - cos;
        let dcosxy = dcos * x * y;
        let dcosxz = dcos * x * z;
        let dcosyz = dcos * y * z;
        let sinx = sin * x;
        let siny = sin * y;
        let sinz = sin * z;
        Self::new(
            [cos + dcos * x * x, dcosxy + sinz, dcosxz - siny, T::ZERO],
            [dcosxy - sinz, cos + dcos * y * y, dcosyz + sinx, T::ZERO],
            [dcosxz + siny, dcosyz - sinx, cos + dcos * z * z, T::ZERO],
            [T::ZERO, T::ZERO, T::ZERO, T::ONE],
        )
    }

    /// Creates a new matrix encoding the rotation described by a given quaternion.
    pub fn rotation_q(quat: &Quat<T>) -> Self {
        // TODO: Implement vector conversions.
        let imag = quat.imag();
        let real = quat.real();
        let qvec = Vec4::new(imag[0], imag[1], imag[2], real).normalize();
        let (x, y, z, w) = (qvec[0], qvec[1], qvec[2], qvec[3]);
        let xx2 = (T::ONE + T::ONE) * x * x;
        let xy2 = (T::ONE + T::ONE) * x * y;
        let xz2 = (T::ONE + T::ONE) * x * z;
        let xw2 = (T::ONE + T::ONE) * x * w;
        let yy2 = (T::ONE + T::ONE) * y * y;
        let yz2 = (T::ONE + T::ONE) * y * z;
        let yw2 = (T::ONE + T::ONE) * y * w;
        let zz2 = (T::ONE + T::ONE) * z * z;
        let zw2 = (T::ONE + T::ONE) * z * w;
        Self::new(
            [T::ONE - yy2 - zz2, xy2 + zw2, xz2 - yw2, T::ZERO],
            [xy2 - zw2, T::ONE - xx2 - zz2, yz2 + xw2, T::ZERO],
            [xz2 + yw2, yz2 - xw2, T::ONE - xx2 - yy2, T::ZERO],
            [T::ZERO, T::ZERO, T::ZERO, T::ONE],
        )
    }

    /// Creates a new matrix encoding a rotation about the x axis.
    pub fn rotation_x(angle: T) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self::new(
            [T::ONE, T::ZERO, T::ZERO, T::ZERO],
            [T::ZERO, cos, sin, T::ZERO],
            [T::ZERO, -sin, cos, T::ZERO],
            [T::ZERO, T::ZERO, T::ZERO, T::ONE],
        )
    }

    /// Creates a new matrix encoding a rotation about the y axis.
    pub fn rotation_y(angle: T) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self::new(
            [cos, T::ZERO, -sin, T::ZERO],
            [T::ZERO, T::ONE, T::ZERO, T::ZERO],
            [sin, T::ZERO, cos, T::ZERO],
            [T::ZERO, T::ZERO, T::ZERO, T::ONE],
        )
    }

    /// Creates a new matrix encoding a rotation about the z axis.
    pub fn rotation_z(angle: T) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self::new(
            [cos, sin, T::ZERO, T::ZERO],
            [-sin, cos, T::ZERO, T::ZERO],
            [T::ZERO, T::ZERO, T::ONE, T::ZERO],
            [T::ZERO, T::ZERO, T::ZERO, T::ONE],
        )
    }
}

impl<T: Scalar> Mat3<T> {
    /// Creates a new matrix encoding a scale.
    pub fn scale(x: T, y: T, z: T) -> Self {
        let mut m = Self::default();
        m[0][0] = x;
        m[1][1] = y;
        m[2][2] = z;
        m
    }
}

impl<T: Scalar> Mat4<T> {
    /// Creates a new matrix encoding a scale.
    pub fn scale(x: T, y: T, z: T) -> Self {
        let mut m = Self::default();
        m[0][0] = x;
        m[1][1] = y;
        m[2][2] = z;
        m[3][3] = T::ONE;
        m
    }
}

impl<T: Float> Mat4<T> {
    /// Creates a new matrix encoding a view transform.
    pub fn look_at(center: &Vec3<T>, eye: &Vec3<T>, up: &Vec3<T>) -> Self {
        let fwd = (center - eye).normalize();
        let side = fwd.cross(up).normalize();
        let up = fwd.cross(&side);
        Self::new(
            [side[0], up[0], -fwd[0], T::ZERO],
            [side[1], up[1], -fwd[1], T::ZERO],
            [side[2], up[2], -fwd[2], T::ZERO],
            [-side.dot(eye), -up.dot(eye), fwd.dot(eye), T::ONE],
        )
    }
}

impl<T: Float> Mat4<T> {
    /// Creates a new matrix encoding a perspective projection.
    pub fn perspective(yfov: T, aspect: T, znear: T, zfar: T) -> Self {
        let two = T::ONE + T::ONE;
        let ct = T::ONE / (yfov / two).tan();
        Self::new(
            [ct / aspect, T::ZERO, T::ZERO, T::ZERO],
            [T::ZERO, ct, T::ZERO, T::ZERO],
            [T::ZERO, T::ZERO, (zfar + znear) / (znear - zfar), -T::ONE],
            [
                T::ZERO,
                T::ZERO,
                (two * zfar * znear) / (znear - zfar),
                T::ZERO,
            ],
        )
    }

    /// Creates a new matrix encoding an infinity perspective projection.
    pub fn inf_perspective(yfov: T, aspect: T, znear: T) -> Self {
        let two = T::ONE + T::ONE;
        let ct = T::ONE / (yfov / two).tan();
        Self::new(
            [ct / aspect, T::ZERO, T::ZERO, T::ZERO],
            [T::ZERO, ct, T::ZERO, T::ZERO],
            [T::ZERO, T::ZERO, -T::ONE, -T::ONE],
            [T::ZERO, T::ZERO, -two * znear, T::ZERO],
        )
    }

    /// Creates a new matrix encoding an orthographic projection.
    pub fn ortho(xmag: T, ymag: T, znear: T, zfar: T) -> Self {
        let two = T::ONE + T::ONE;
        Self::new(
            [T::ONE / xmag, T::ZERO, T::ZERO, T::ZERO],
            [T::ZERO, T::ONE / ymag, T::ZERO, T::ZERO],
            [T::ZERO, T::ZERO, two / (znear - zfar), T::ZERO],
            [T::ZERO, T::ZERO, (zfar + znear) / (znear - zfar), T::ONE],
        )
    }
}

macro_rules! conv_impl {
    ($m:ty, $v:ty, $n:literal) => {
        // NOTE: `Scalar` bounded due to type inference.
        impl<T: Scalar> From<T> for $m {
            /// Converts a scalar into a matrix whose diagonal contains copies
            /// of such scalar. Non-diagonal components are set to the default value
            /// (which is expected to be `T::ZERO`).
            ///
            /// This conversion can be used to create identity matrices with the
            /// convenience of type inference.
            fn from(diag: T) -> Self {
                let mut m = Self::default();
                for i in 0..$n {
                    m[i][i] = diag;
                }
                m
            }
        }

        impl<T: Copy + Default> From<&$v> for $m {
            /// Converts a vector into a matrix whose diagonal contains the components
            /// of such vector. Non-diagonal components are set to the default value.
            fn from(diag: &$v) -> Self {
                let mut m = Self::default();
                for i in 0..$n {
                    m[i][i] = diag[i];
                }
                m
            }
        }

        impl<T: Copy + Default> From<$v> for $m {
            /// Converts a vector into a matrix whose diagonal contains the components
            /// of such vector. Non-diagonal components are set to the default value.
            fn from(diag: $v) -> Self {
                <$m>::from(&diag)
            }
        }

        impl<T: Copy + Default> From<&[T; $n * $n]> for $m {
            /// Converts an array into a matrix.
            fn from(array: &[T; $n * $n]) -> Self {
                let mut m = Self::default();
                for i in 0..$n {
                    for j in 0..$n {
                        m[i][j] = array[i * $n + j];
                    }
                }
                m
            }
        }

        impl<T: Default> From<[T; $n * $n]> for $m {
            /// Converts an array into a matrix.
            fn from(mut array: [T; $n * $n]) -> Self {
                let mut m = Self::default();
                for i in 0..$n {
                    for j in 0..$n {
                        m[i][j] = mem::take(&mut array[i * $n + j]);
                    }
                }
                m
            }
        }
    };
}

conv_impl!(Mat2<T>, Vec2<T>, 2);
conv_impl!(Mat3<T>, Vec3<T>, 3);
conv_impl!(Mat4<T>, Vec4<T>, 4);

impl<T: Scalar> From<&Mat3<T>> for Mat4<T> {
    /// Converts a `&Mat3<T>` into an homogeneous `Mat4<T>`.
    fn from(upper_left: &Mat3<T>) -> Self {
        let mut m = Self::default();
        for i in 0..3 {
            for j in 0..3 {
                m[i][j] = upper_left[i][j];
            }
        }
        m[3][3] = T::ONE;
        m
    }
}

impl<T: Scalar> From<Mat3<T>> for Mat4<T> {
    /// Converts a `Mat3<T>` into an homogeneous `Mat4<T>`.
    fn from(upper_left: Mat3<T>) -> Self {
        let mut m = Self::default();
        for i in 0..3 {
            for j in 0..3 {
                m[i][j] = upper_left[i][j];
            }
        }
        m[3][3] = T::ONE;
        m
    }
}

impl<T: Copy + Default> From<&Mat4<T>> for Mat3<T> {
    /// Converts the upper-left of a `&Mat4<T>` into a `Mat3<T>`.
    fn from(upper_left: &Mat4<T>) -> Self {
        let mut m = Self::default();
        for i in 0..3 {
            for j in 0..3 {
                m[i][j] = upper_left[i][j];
            }
        }
        m
    }
}

impl<T: Copy + Default> From<Mat4<T>> for Mat3<T> {
    /// Converts the upper-left of a `Mat4<T>` into a `Mat3<T>`.
    fn from(upper_left: Mat4<T>) -> Self {
        let mut m = Self::default();
        for i in 0..3 {
            for j in 0..3 {
                m[i][j] = upper_left[i][j];
            }
        }
        m
    }
}

impl<T: Float> Mat4<T> {
    /// Composes a matrix from TRS properties.
    pub fn from_trs(t: &Vec3<T>, r: &Quat<T>, s: &Vec3<T>) -> Self {
        let mut m = Mat4::rotation_q(r);
        m[0][0] *= s[0];
        m[0][1] *= s[0];
        m[0][2] *= s[0];
        m[1][0] *= s[1];
        m[1][1] *= s[1];
        m[1][2] *= s[1];
        m[2][0] *= s[2];
        m[2][1] *= s[2];
        m[2][2] *= s[2];
        m[3][0] = t[0];
        m[3][1] = t[1];
        m[3][2] = t[2];
        m
    }

    /// Composes a matrix from a translation vector.
    pub fn from_t(t: &Vec3<T>) -> Self {
        Mat4::translation(t[0], t[1], t[2])
    }

    /// Composes a matrix from a rotation quaternion.
    pub fn from_r(r: &Quat<T>) -> Self {
        Mat4::rotation_q(r)
    }

    /// Composes a matrix from a scale vector.
    pub fn from_s(s: &Vec3<T>) -> Self {
        Mat4::scale(s[0], s[1], s[2])
    }

    /// Decomposes a matrix into TRS properties.
    pub fn into_trs(&self) -> (Vec3<T>, Quat<T>, Vec3<T>) {
        let mut ul = Mat3::from(self);
        let det = ul.det();
        let s = if det > T::ZERO {
            Vec3::new(ul[0].length(), ul[1].length(), ul[2].length())
        } else {
            Vec3::new(-ul[0].length(), -ul[1].length(), -ul[2].length())
        };
        let r = if det.abs() <= T::EPSILON {
            Quat::new([T::ZERO; 3], T::ONE)
        } else {
            ul[0] /= s[0];
            ul[1] /= s[1];
            ul[2] /= s[2];
            Quat::rotation_m(&ul)
        };
        let t = Vec3::from(self[3]);
        (t, r, s)
    }

    /// Decomposes a matrix into a translation vector.
    pub fn into_t(&self) -> Vec3<T> {
        Vec3::from(self[3])
    }

    /// Decomposes a matrix into a rotation quaternion.
    pub fn into_r(&self) -> Quat<T> {
        let mut ul = Mat3::from(self);
        let det = ul.det();
        let s = if det > T::ZERO {
            Vec3::new(ul[0].length(), ul[1].length(), ul[2].length())
        } else {
            Vec3::new(-ul[0].length(), -ul[1].length(), -ul[2].length())
        };
        if det.abs() <= T::EPSILON {
            Quat::new([T::ZERO; 3], T::ONE)
        } else {
            ul[0] /= s[0];
            ul[1] /= s[1];
            ul[2] /= s[2];
            Quat::rotation_m(&ul)
        }
    }

    /// Decomposes a matrix into a scale vector.
    pub fn into_s(&self) -> Vec3<T> {
        let ul = Mat3::from(self);
        if ul.det() > T::ZERO {
            Vec3::new(ul[0].length(), ul[1].length(), ul[2].length())
        } else {
            Vec3::new(-ul[0].length(), -ul[1].length(), -ul[2].length())
        }
    }
}
