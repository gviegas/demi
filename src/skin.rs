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
    prev_slot: Option<u16>,
    jm: Mat4<f32>,
    ibm: Option<Mat4<f32>>,
    name: String,
}

impl Joint {
    /// Returns the slot containing the parent of this joint,
    /// or `None` if there is no previous joint.
    ///
    /// NOTE: This does not mean that this is a root joint.
    pub fn prev_slot(&self) -> Option<u16> {
        self.prev_slot
    }

    /// Returns a reference to the joint's matrix.
    pub fn joint_matrix(&self) -> &Mat4<f32> {
        &self.jm
    }

    /// Returns a reference to the inverse bind matrix,
    /// or `None` if it is the identity.
    pub fn inverse_bind_matrix(&self) -> Option<&Mat4<f32>> {
        self.ibm.as_ref()
    }

    /// Returns the name of this joint.
    pub fn name(&self) -> &str {
        &self.name
    }
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

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}
