// Copyright 2022 Gustavo C. Viegas. All rights reserved.

//! Blend-weight skinning.

use std::io;

use crate::linear::Mat4;

/// Skin.
#[derive(Debug)]
pub struct Skin(Vec<Joint>);

impl Skin {
    /// Returns a reference to the skin's [`Joint`]s.
    pub fn joints(&self) -> &[Joint] {
        &self.0
    }
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
    /// NOTE: This only pertains to direct connections between
    /// skin joints. Skins sourced from external node graphs
    /// may contain joint nodes that are indirectly connected
    /// through other nodes. Such relations are not preserved.
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
    /// Creates a new skin builder.
    pub fn new() -> Self {
        todo!();
    }

    /// Pushes a number of joints.
    ///
    /// A new joint is created for every element of the
    /// slice parameters, in order.
    /// All slices must have the same length.
    ///
    /// This method fails if the total number of joints
    /// exceeds [`u16::MAX`] across all `push_joints`
    /// calls for a single skin.
    pub fn push_joints(
        &mut self,
        prev_slot: &[Option<u16>],
        jm: &[Mat4<f32>],
        ibm: &[Option<Mat4<f32>>],
        name: &[String],
    ) -> io::Result<&mut Self> {
        todo!();
    }

    /// Creates the skin.
    ///
    /// This method consumes every pushed joint to create
    /// the skin. The order which the joints were pushed
    /// will be used to identify its slot in the [`Skin`].
    ///
    /// Fails if no joint has been pushed yet.
    pub fn create(&mut self) -> io::Result<Skin> {
        todo!();
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}
