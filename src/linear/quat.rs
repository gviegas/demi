// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use std::ops::{Add, Mul, MulAssign, Sub};

use crate::linear::Vec3;

/// Quaternion.
#[derive(Copy, Clone, Debug)]
pub struct Quat<T>(Vec3<T>, T);

impl<T: Copy> Quat<T> {
    pub fn new(i: &[T; 3], r: T) -> Self {
        Self(Vec3::new(&i), r)
    }

    pub fn imag(&self) -> Vec3<T> {
        self.0
    }

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
        // TODO: Implement ops for VecN values too.
        Quat::<T>(
            &(&(i1 * r2) + &(i2 * r1)) + &i1.cross(i2),
            r1 * r2 - i1.dot(i2),
        )
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
        *self = Quat::<T>(
            &(&(i1 * r2) + &(i2 * r1)) + &i1.cross(i2),
            r1 * r2 - i1.dot(i2),
        );
        */
    }
}

// NOTE: Floating-point only.
macro_rules! rotation_impl {
    ($f:ty, $zero:literal, $half:literal) => {
        impl Quat<$f> {
            pub fn rotation(angle: $f, axis: &Vec3<$f>) -> Self {
                let ang = angle * $half;
                let cos = ang.cos();
                let sin = ang.sin();
                Self(&axis.norm() * sin, cos)
            }

            pub fn rotation_x(angle: $f) -> Self {
                let ang = angle * $half;
                let cos = ang.cos();
                let sin = ang.sin();
                Self(Vec3::new(&[sin, $zero, $zero]), cos)
            }

            pub fn rotation_y(angle: $f) -> Self {
                let ang = angle * $half;
                let cos = ang.cos();
                let sin = ang.sin();
                Self(Vec3::new(&[$zero, sin, $zero]), cos)
            }

            pub fn rotation_z(angle: $f) -> Self {
                let ang = angle * $half;
                let cos = ang.cos();
                let sin = ang.sin();
                Self(Vec3::new(&[$zero, $zero, sin]), cos)
            }
        }
    };
}

rotation_impl!(f32, 0f32, 0.5f32);
rotation_impl!(f64, 0f64, 0.5f64);
