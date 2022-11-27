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
