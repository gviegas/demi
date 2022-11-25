// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

/// 2-component vector.
#[derive(Copy, Clone, Debug)]
pub struct Vec2<T>([T; 2]);

/// 3-component vector.
#[derive(Copy, Clone, Debug)]
pub struct Vec3<T>([T; 3]);

/// 4-component vector.
#[derive(Copy, Clone, Debug)]
pub struct Vec4<T>([T; 4]);

macro_rules! new_impl {
    ($t:ty, $n:literal) => {
        impl<T: Copy> $t {
            pub fn new(v: &[T; $n]) -> Self {
                Self(*v)
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
                let mut v = <$t>::new(&[T::default(); $n]);
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
                let mut v = <$t>::new(&[T::default(); $n]);
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
        impl<T: Copy + Default + SubAssign> SubAssign<&$t> for $t {
            fn sub_assign(&mut self, other: &Self) {
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
                <$t>::new(&self.0.map(|x| x * scalar))
            }
        }
    };
}

mul_impl!(Vec2<T>);
mul_impl!(Vec3<T>);
mul_impl!(Vec4<T>);

macro_rules! mul_assign_impl {
    ($t:ty) => {
        impl<T: Copy + Default + MulAssign> MulAssign<T> for $t {
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
                <$t>::new(&self.0.map(|x| x / scalar))
            }
        }
    };
}

div_impl!(Vec2<T>);
div_impl!(Vec3<T>);
div_impl!(Vec4<T>);

macro_rules! div_assign_impl {
    ($t:ty) => {
        impl<T: Copy + Default + DivAssign> DivAssign<T> for $t {
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
    };
}

neg_impl!(Vec2<T>);
neg_impl!(Vec3<T>);
neg_impl!(Vec4<T>);
