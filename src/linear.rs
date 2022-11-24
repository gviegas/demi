// Copyright 2022 Gustavo C. Viegas. All rights reserved.

/// 3-component vector.
#[derive(Copy, Clone, Debug)]
pub struct Vec3<T>([T; 3]);

/// 4-component vector.
#[derive(Copy, Clone, Debug)]
pub struct Vec4<T>([T; 4]);

/// Column-major 3x3 matrix.
#[derive(Copy, Clone, Debug)]
pub struct Mat3<T>([Vec3<T>; 3]);

/// Column-major 4x4 matrix.
#[derive(Copy, Clone, Debug)]
pub struct Mat4<T>([Vec4<T>; 4]);

/// Unit quaternion.
#[derive(Copy, Clone, Debug)]
pub struct Quat<T>(Vec4<T>);

impl<T: Copy> Vec3<T> {
    pub fn new(v: &[T; 3]) -> Self {
        Self(*v)
    }
}

impl<T: Copy> Vec4<T> {
    pub fn new(v: &[T; 4]) -> Self {
        Self(*v)
    }
}

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

impl<T: Copy> Quat<T> {
    pub fn new(v: &Vec3<T>, r: T) -> Self {
        Self(Vec4::new(&[v.0[0], v.0[1], v.0[2], r]))
    }
}

// TODO...
