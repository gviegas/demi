// Copyright 2022 Gustavo C. Viegas. All rights reserved.

//! Utilities for 3D math.

use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

mod vecn;
pub use crate::linear::vecn::{Vec2, Vec3, Vec4};

mod matn;
pub use crate::linear::matn::{Mat2, Mat3, Mat4};

mod quat;
pub use crate::linear::quat::Quat;

#[cfg(test)]
mod tests;

/// Number.
pub trait Scalar:
    Copy
    + Default
    + PartialOrd
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Rem<Output = Self>
    + RemAssign
{
    const ZERO: Self;
    const ONE: Self;
}

// NOTE: Primitives only.
macro_rules! integer_impl {
    ($($t:ty),*) => {$(
        impl Scalar for $t {
            const ZERO: $t = 0;
            const ONE: $t = 1;
        }
    )*};
}

integer_impl!(i8, i16, i32, i64, u8, u16, u32, u64);

/// Real number.
pub trait Float: Scalar + Neg<Output = Self> {
    const EPSILON: Self;

    fn round(self) -> Self;
    fn trunc(self) -> Self;
    fn ceil(self) -> Self;
    fn floor(self) -> Self;
    fn abs(self) -> Self;
    fn sqrt(self) -> Self;
    fn cos(self) -> Self;
    fn sin(self) -> Self;
    fn tan(self) -> Self;
}

// NOTE: Primitives only.
macro_rules! noninteger_impl {
    ($($t:ty),*) => {$(
        impl Scalar for $t {
            const ZERO: $t = 0.0;
            const ONE: $t = 1.0;
        }

        impl Float for $t {
            const EPSILON: $t = <$t>::EPSILON;

            fn round(self) -> Self {
                self.round()
            }

            fn trunc(self) -> Self {
                self.trunc()
            }

            fn ceil(self) -> Self {
                self.ceil()
            }

            fn floor(self) -> Self {
                self.floor()
            }

            fn abs(self) -> Self {
                self.abs()
            }

            fn sqrt(self) -> Self {
                self.sqrt()
            }

            fn cos(self) -> Self {
                self.cos()
            }

            fn sin(self) -> Self {
                self.sin()
            }

            fn tan(self) -> Self {
                self.tan()
            }
        }
    )*};
}

noninteger_impl!(f32, f64);
