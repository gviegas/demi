// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use std::ops::{Add, Mul, MulAssign, Sub};

use crate::linear::{Float, Vec3, Vec4};

/// Quaternion.
#[derive(Copy, Clone, Debug)]
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

impl<T: Copy + Default> From<&Vec4<T>> for Quat<T> {
    /// Converts a `&Vec4<T>` into a `Quat<T>`.
    ///
    /// The real part is taken from the last component of the vector.
    fn from(iiir: &Vec4<T>) -> Self {
        Self(Vec3::new([iiir[0], iiir[1], iiir[2]]), iiir[3])
    }
}

impl<T: Copy + Default> From<Vec4<T>> for Quat<T> {
    /// Converts a `Vec4<T>` into a `Quat<T>`.
    ///
    /// The real part is taken from the last component of the vector.
    fn from(iiir: Vec4<T>) -> Self {
        Self(Vec3::new([iiir[0], iiir[1], iiir[2]]), iiir[3])
    }
}
