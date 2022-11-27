// Copyright 2022 Gustavo C. Viegas. All rights reserved.

mod vecn;
pub use crate::linear::vecn::{Vec2, Vec3, Vec4};

mod matn;
pub use crate::linear::matn::{Mat2, Mat3, Mat4};

mod quat;
pub use crate::linear::quat::Quat;

#[cfg(test)]
mod tests;
