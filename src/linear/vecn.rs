// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use crate::linear::{Float, Mat2, Mat3, Mat4, Quat, Scalar};

/// 2-component vector.
#[derive(Copy, Clone, Default, Debug)]
pub struct Vec2<T>([T; 2]);

/// 3-component vector.
#[derive(Copy, Clone, Default, Debug)]
pub struct Vec3<T>([T; 3]);

/// 4-component vector.
#[derive(Copy, Clone, Default, Debug)]
pub struct Vec4<T>([T; 4]);

macro_rules! new_impl {
    ($t:ty, $n:literal) => {
        impl<T: Copy> $t {
            /// Creates a new vector from an array of values.
            pub fn new(v: [T; $n]) -> Self {
                Self(v)
            }
        }
    };
}

new_impl!(Vec2<T>, 2);
new_impl!(Vec3<T>, 3);
new_impl!(Vec4<T>, 4);

macro_rules! index_impl {
    ($t:ty) => {
        impl<T> Index<usize> for $t {
            type Output = T;

            fn index(&self, index: usize) -> &Self::Output {
                &self.0[index]
            }
        }

        impl<T> IndexMut<usize> for $t {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.0[index]
            }
        }
    };
}

index_impl!(Vec2<T>);
index_impl!(Vec3<T>);
index_impl!(Vec4<T>);

macro_rules! add_impl {
    ($t:ty, $n:literal) => {
        impl<T: Copy + Default + Add<Output = T>> Add for &$t {
            type Output = $t;

            fn add(self, other: Self) -> Self::Output {
                let mut v = <$t>::default();
                for i in 0..$n {
                    v[i] = self[i] + other[i];
                }
                v
            }
        }

        impl<T: Copy + Default + Add<Output = T>> Add for $t {
            type Output = Self;

            fn add(self, other: Self) -> Self::Output {
                let mut v = Self::default();
                for i in 0..$n {
                    v[i] = self[i] + other[i];
                }
                v
            }
        }
    };
}

add_impl!(Vec2<T>, 2);
add_impl!(Vec3<T>, 3);
add_impl!(Vec4<T>, 4);

macro_rules! add_assign_impl {
    ($t:ty, $n:literal) => {
        impl<T: Copy + AddAssign> AddAssign<&$t> for $t {
            fn add_assign(&mut self, other: &Self) {
                for i in 0..$n {
                    self[i] += other[i];
                }
            }
        }

        impl<T: Copy + AddAssign> AddAssign for $t {
            fn add_assign(&mut self, other: Self) {
                for i in 0..$n {
                    self[i] += other[i];
                }
            }
        }
    };
}

add_assign_impl!(Vec2<T>, 2);
add_assign_impl!(Vec3<T>, 3);
add_assign_impl!(Vec4<T>, 4);

macro_rules! sub_impl {
    ($t:ty, $n:literal) => {
        impl<T: Copy + Default + Sub<Output = T>> Sub for &$t {
            type Output = $t;

            fn sub(self, other: Self) -> Self::Output {
                let mut v = <$t>::default();
                for i in 0..$n {
                    v[i] = self[i] - other[i];
                }
                v
            }
        }

        impl<T: Copy + Default + Sub<Output = T>> Sub for $t {
            type Output = Self;

            fn sub(self, other: Self) -> Self::Output {
                let mut v = Self::default();
                for i in 0..$n {
                    v[i] = self[i] - other[i];
                }
                v
            }
        }
    };
}

sub_impl!(Vec2<T>, 2);
sub_impl!(Vec3<T>, 3);
sub_impl!(Vec4<T>, 4);

macro_rules! sub_assign_impl {
    ($t:ty, $n:literal) => {
        impl<T: Copy + SubAssign> SubAssign<&$t> for $t {
            fn sub_assign(&mut self, other: &Self) {
                for i in 0..$n {
                    self[i] -= other[i];
                }
            }
        }

        impl<T: Copy + SubAssign> SubAssign for $t {
            fn sub_assign(&mut self, other: Self) {
                for i in 0..$n {
                    self[i] -= other[i];
                }
            }
        }
    };
}

sub_assign_impl!(Vec2<T>, 2);
sub_assign_impl!(Vec3<T>, 3);
sub_assign_impl!(Vec4<T>, 4);

macro_rules! mul_impl {
    ($t:ty) => {
        impl<T: Copy + Default + Mul<Output = T>> Mul<T> for &$t {
            type Output = $t;

            fn mul(self, scalar: T) -> Self::Output {
                // TODO: Compare to a simple for loop.
                <$t>::new(self.0.map(|x| x * scalar))
            }
        }

        impl<T: Copy + Default + Mul<Output = T>> Mul<T> for $t {
            type Output = Self;

            fn mul(self, scalar: T) -> Self::Output {
                // TODO: Compare to a simple for loop.
                Self::new(self.0.map(|x| x * scalar))
            }
        }
    };
}

mul_impl!(Vec2<T>);
mul_impl!(Vec3<T>);
mul_impl!(Vec4<T>);

macro_rules! mul_assign_impl {
    ($t:ty) => {
        impl<T: Copy + MulAssign> MulAssign<T> for $t {
            fn mul_assign(&mut self, scalar: T) {
                for i in &mut self.0 {
                    *i *= scalar;
                }
            }
        }
    };
}

mul_assign_impl!(Vec2<T>);
mul_assign_impl!(Vec3<T>);
mul_assign_impl!(Vec4<T>);

macro_rules! div_impl {
    ($t:ty) => {
        impl<T: Copy + Default + Div<Output = T>> Div<T> for &$t {
            type Output = $t;

            fn div(self, scalar: T) -> Self::Output {
                // TODO: Compare to a simple for loop.
                <$t>::new(self.0.map(|x| x / scalar))
            }
        }

        impl<T: Copy + Default + Div<Output = T>> Div<T> for $t {
            type Output = Self;

            fn div(self, scalar: T) -> Self::Output {
                // TODO: Compare to a simple for loop.
                Self::new(self.0.map(|x| x / scalar))
            }
        }
    };
}

div_impl!(Vec2<T>);
div_impl!(Vec3<T>);
div_impl!(Vec4<T>);

macro_rules! div_assign_impl {
    ($t:ty) => {
        impl<T: Copy + DivAssign> DivAssign<T> for $t {
            fn div_assign(&mut self, scalar: T) {
                for i in &mut self.0 {
                    *i /= scalar;
                }
            }
        }
    };
}

div_assign_impl!(Vec2<T>);
div_assign_impl!(Vec3<T>);
div_assign_impl!(Vec4<T>);

macro_rules! neg_impl {
    ($t:ty) => {
        impl<T: Copy + Neg<Output = T>> Neg for &$t {
            type Output = $t;

            fn neg(self) -> Self::Output {
                let mut v = *self;
                for i in &mut v.0 {
                    *i = -*i;
                }
                v
            }
        }

        impl<T: Copy + Neg<Output = T>> Neg for $t {
            type Output = Self;

            fn neg(self) -> Self::Output {
                let mut v = self;
                for i in &mut v.0 {
                    *i = -*i;
                }
                v
            }
        }
    };
}

neg_impl!(Vec2<T>);
neg_impl!(Vec3<T>);
neg_impl!(Vec4<T>);

macro_rules! dot_impl {
    ($t:ty) => {
        impl<T: Copy + Default + Add<Output = T> + Mul<Output = T>> $t {
            /// Computes the dot product.
            pub fn dot(&self, other: &Self) -> T {
                // TODO: Compare to a simple for loop.
                self.0
                    .iter()
                    .zip(&other.0)
                    .fold(T::default(), |acc, (a, b)| acc + *a * *b)
            }
        }
    };
}

dot_impl!(Vec2<T>);
dot_impl!(Vec3<T>);
dot_impl!(Vec4<T>);

macro_rules! length_impl {
    ($t:ty) => {
        impl<T: Float> $t {
            /// Computes the vector's length.
            pub fn length(&self) -> T {
                // TODO: Compare to a simple for loop.
                self.0
                    .iter()
                    .zip(&self.0)
                    .fold(T::default(), |acc, (a, b)| acc + *a * *b)
                    .sqrt()

                // NOTE: This would require `Float` bound on `dot`.
                //self.dot(self).sqrt()
            }
        }
    };
}

length_impl!(Vec2<T>);
length_impl!(Vec3<T>);
length_impl!(Vec4<T>);

macro_rules! norm_impl {
    ($t:ty) => {
        impl<T: Float> $t {
            /// Returns a new direction vector.
            ///
            /// NOTE: One must ensure that `self.length()` is greater than zero.
            #[must_use]
            pub fn norm(&self) -> Self {
                self / self.length()
            }
        }
    };
}

norm_impl!(Vec2<T>);
norm_impl!(Vec3<T>);
norm_impl!(Vec4<T>);

impl<T: Copy + Sub<Output = T> + Mul<Output = T>> Vec3<T> {
    /// Computes the cross product.
    #[must_use]
    pub fn cross(&self, other: &Self) -> Self {
        Self([
            self[1] * other[2] - other[1] * self[2],
            self[2] * other[0] - other[2] * self[0],
            self[0] * other[1] - other[0] * self[1],
        ])
    }
}

macro_rules! conv_impl {
    ($v:ty, $m:ty, $n:literal) => {
        // NOTE: `Scalar` bounded due to type inference.
        impl<T: Scalar> From<T> for $v {
            /// Converts a scalar into a vector whose components are copies
            /// of such scalar.
            fn from(value: T) -> Self {
                Self([value; $n])
            }
        }

        impl<T: Copy + Default> From<&$m> for $v {
            /// Converts a matrix's diagonal into a vector.
            fn from(diag: &$m) -> Self {
                let mut v = Self::default();
                for i in 0..$n {
                    v[i] = diag[i][i];
                }
                v
            }
        }

        impl<T: Copy + Default> From<$m> for $v {
            /// Converts a matrix's diagonal into a vector.
            fn from(diag: $m) -> Self {
                <$v>::from(&diag)
            }
        }
    };
}

conv_impl!(Vec2<T>, Mat2<T>, 2);
conv_impl!(Vec3<T>, Mat3<T>, 3);
conv_impl!(Vec4<T>, Mat4<T>, 4);

impl<T: Copy + Default> From<&Quat<T>> for Vec4<T> {
    /// Converts a `&Quat<T>` into a `Vec4<T>`.
    ///
    /// The real part is stored in the last component of the vector.
    fn from(iiir: &Quat<T>) -> Self {
        let i = iiir.imag();
        let r = iiir.real();
        Self([i[0], i[1], i[2], r])
    }
}

impl<T: Copy + Default> From<Quat<T>> for Vec4<T> {
    /// Converts a `Quat<T>` into a `Vec4<T>`.
    ///
    /// The real part is stored in the last component of the vector.
    fn from(iiir: Quat<T>) -> Self {
        Self::from(&iiir)
        //let i = iiir.imag();
        //let r = iiir.real();
        //Self([i[0], i[1], i[2], r])
    }
}
