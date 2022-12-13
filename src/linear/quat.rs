// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use std::ops::{Add, Mul, MulAssign, Sub};

use crate::linear::{Float, Mat3, Vec3, Vec4};

/// Quaternion.
#[derive(Copy, Clone, Default, Debug)]
pub struct Quat<T>(Vec3<T>, T);

impl<T: Copy> Quat<T> {
    /// Creates a new quaternion from a vector (imaginary part) and
    /// a value (real part).
    pub fn new(i: [T; 3], r: T) -> Self {
        Self(Vec3::new(i), r)
    }

    /// Returns the imaginary part.
    pub fn imag(&self) -> Vec3<T> {
        self.0
    }

    /// Returns the real part.
    pub fn real(&self) -> T {
        self.1
    }
}

impl<T> Mul for &Quat<T>
where
    T: Copy + Default + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    type Output = Quat<T>;

    fn mul(self, other: Self) -> Self::Output {
        let i1 = &self.0;
        let r1 = self.1;
        let i2 = &other.0;
        let r2 = other.1;
        Quat::<T>(i1 * r2 + i2 * r1 + i1.cross(i2), r1 * r2 - i1.dot(i2))
    }
}

impl<T> Mul for Quat<T>
where
    T: Copy + Default + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let i1 = &self.0;
        let r1 = self.1;
        let i2 = &other.0;
        let r2 = other.1;
        Quat::<T>(i1 * r2 + i2 * r1 + i1.cross(i2), r1 * r2 - i1.dot(i2))
    }
}

impl<T> MulAssign<&Quat<T>> for Quat<T>
where
    T: Copy + Default + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    fn mul_assign(&mut self, other: &Self) {
        *self = &*self * other;
        /*
        let i1 = &self.0;
        let r1 = self.1;
        let i2 = &other.0;
        let r2 = other.1;
        *self = Quat::<T>(i1 * r2 + i2 * r1 + i1.cross(i2), r1 * r2 - i1.dot(i2));
        */
    }
}

impl<T> MulAssign for Quat<T>
where
    T: Copy + Default + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl<T: Float> Quat<T> {
    /// Creates a new quaternion encoding a rotation about an arbitrary axis.
    pub fn rotation(angle: T, axis: &Vec3<T>) -> Self {
        let ang = angle / (T::ONE + T::ONE);
        let cos = ang.cos();
        let sin = ang.sin();
        Self(axis.norm() * sin, cos)
    }

    /// Creates a new quaternion encoding the rotation described by a given matrix.
    pub fn rotation_m(mat: &Mat3<T>) -> Self {
        let diag = Vec3::from(mat);
        match diag[0] + diag[1] + diag[2] {
            x if x > T::ZERO => {
                let s = (T::ONE + x).sqrt();
                Self(
                    Vec3::new([
                        mat[1][2] - mat[2][1],
                        mat[2][0] - mat[0][2],
                        mat[0][1] - mat[1][0],
                    ]) * (T::ONE / (s + s)),
                    s / (T::ONE + T::ONE),
                )
            }
            _ => {
                let (i, j, k) = if diag[0] > diag[1] {
                    if diag[1] > diag[2] {
                        (0, 1, 2)
                    } else if diag[2] > diag[0] {
                        (2, 0, 1)
                    } else {
                        (0, 2, 1)
                    }
                } else if diag[1] > diag[2] {
                    if diag[2] > diag[0] {
                        (1, 2, 0)
                    } else {
                        (1, 0, 2)
                    }
                } else {
                    (2, 1, 0)
                };
                let s = (diag[i] - (diag[j] + diag[k]) + T::ONE).sqrt();
                debug_assert!(s.abs() > T::EPSILON);
                let is = T::ONE / (s + s);
                let mut v = Vec3::default();
                v[i] = s / (T::ONE + T::ONE);
                v[j] = is * (mat[i][j] + mat[j][i]);
                v[k] = is * (mat[i][k] + mat[k][i]);
                Self(v, is * (mat[j][k] - mat[k][j]))
            }
        }
    }

    /// Creates a new quaternion encoding a rotation about the x axis.
    pub fn rotation_x(angle: T) -> Self {
        let ang = angle / (T::ONE + T::ONE);
        let cos = ang.cos();
        let sin = ang.sin();
        Self(Vec3::new([sin, T::ZERO, T::ZERO]), cos)
    }

    /// Creates a new quaternion encoding a rotation about the y axis.
    pub fn rotation_y(angle: T) -> Self {
        let ang = angle / (T::ONE + T::ONE);
        let cos = ang.cos();
        let sin = ang.sin();
        Self(Vec3::new([T::ZERO, sin, T::ZERO]), cos)
    }

    /// Creates a new quaternion encoding a rotation about the z axis.
    pub fn rotation_z(angle: T) -> Self {
        let ang = angle / (T::ONE + T::ONE);
        let cos = ang.cos();
        let sin = ang.sin();
        Self(Vec3::new([T::ZERO, T::ZERO, sin]), cos)
    }
}

impl<T: Copy> From<&Vec4<T>> for Quat<T> {
    /// Converts a `&Vec4<T>` into a `Quat<T>`.
    ///
    /// The real part is taken from the last component of the vector.
    fn from(ir: &Vec4<T>) -> Self {
        Self(Vec3::new([ir[0], ir[1], ir[2]]), ir[3])
    }
}

impl<T: Copy> From<Vec4<T>> for Quat<T> {
    /// Converts a `Vec4<T>` into a `Quat<T>`.
    ///
    /// The real part is taken from the last component of the vector.
    fn from(ir: Vec4<T>) -> Self {
        Self(Vec3::new([ir[0], ir[1], ir[2]]), ir[3])
    }
}
