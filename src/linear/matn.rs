// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use crate::linear::{Vec3, Vec4};

/// Column-major 3x3 matrix.
#[derive(Copy, Clone, Debug)]
pub struct Mat3<T>([Vec3<T>; 3]);

/// Column-major 4x4 matrix.
#[derive(Copy, Clone, Debug)]
pub struct Mat4<T>([Vec4<T>; 4]);

impl<T: Copy> Mat3<T> {
    pub fn new(col0: &Vec3<T>, col1: &Vec3<T>, col2: &Vec3<T>) -> Self {
        Self([*col0, *col1, *col2])
    }
}

impl<T: Copy> Mat4<T> {
    pub fn new(col0: &Vec4<T>, col1: &Vec4<T>, col2: &Vec4<T>, col3: &Vec4<T>) -> Self {
        Self([*col0, *col1, *col2, *col3])
    }
}
