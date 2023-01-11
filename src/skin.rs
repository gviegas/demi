// Copyright 2022 Gustavo C. Viegas. All rights reserved.

//! Blend-weight skinning.

use std::io;

use crate::linear::Mat4;

/// Skin.
#[derive(Debug)]
pub struct Skin {
    // TODO
}

/// Skin joint.
#[derive(Debug)]
pub struct Joint {
    // TODO
}

/// Skin builder.
pub struct Builder {
    // TODO
}

#[allow(unused_variables)] // TODO
impl Builder {
    pub fn new() -> Self {
        todo!();
    }

    pub fn set_joint_count(&mut self, count: u16) -> &mut Self {
        todo!();
    }

    pub fn set_joint(
        &mut self,
        slot: usize,
        jm: &Mat4<f32>,
        ibm: Option<&Mat4<f32>>,
        name: &str,
    ) -> &mut Self {
        todo!()
    }

    pub fn push_joints(
        &mut self,
        jm: &[Mat4<f32>],
        ibm: Option<&[Mat4<f32>]>,
        name: &[&str],
    ) -> &mut Self {
        todo!();
    }

    pub fn create(&mut self) -> io::Result<Skin> {
        todo!();
    }
}
