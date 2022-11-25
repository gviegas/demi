// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use crate::linear::{Vec3, Vec4};

/// Quaternion.
#[derive(Copy, Clone, Debug)]
pub struct Quat<T>(Vec4<T>);

impl<T: Copy> Quat<T> {
    pub fn new(v: &Vec3<T>, r: T) -> Self {
        Self(Vec4::new(&[v[0], v[1], v[2], r]))
    }
}
