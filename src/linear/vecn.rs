// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use std::ops::Index;

/// 3-component vector.
#[derive(Copy, Clone, Debug)]
pub struct Vec3<T>([T; 3]);

/// 4-component vector.
#[derive(Copy, Clone, Debug)]
pub struct Vec4<T>([T; 4]);

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

macro_rules! def_common {
    ($t:ty) => {
        impl<T> Index<usize> for $t {
            type Output = T;

            fn index(&self, index: usize) -> &Self::Output {
                &self.0[index]
            }
        }
    };
}

def_common!(Vec3<T>);
def_common!(Vec4<T>);
