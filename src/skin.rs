// Copyright 2022 Gustavo C. Viegas. All rights reserved.

//! Blend-weight skinning.

// TODO: Skin instancing; joint hierarchy construction.

use std::io;
use std::mem;

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
    /// may contain joint nodes that are connected indirectly
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
pub struct Builder(Vec<Joint>);

#[allow(unused_variables)] // TODO
impl Builder {
    /// Creates a new skin builder.
    pub fn new() -> Self {
        Self(vec![])
    }

    /// Pushes a number of joints.
    ///
    /// A new joint is created for every element of the
    /// slice parameters, in order.
    /// All slices must have the same length.
    ///
    /// This method will fail if the total number of joints
    /// exceeds [`u16::MAX`] across all `push_joints`
    /// calls for a single skin.
    pub fn push_joints(
        &mut self,
        prev_slot: &[Option<u16>],
        jm: &[Mat4<f32>],
        ibm: &[Option<Mat4<f32>>],
        name: &[&str],
    ) -> io::Result<&mut Self> {
        if prev_slot.len() != jm.len() || jm.len() != ibm.len() || ibm.len() != name.len() {
            Err(io::Error::from(io::ErrorKind::InvalidInput))
        } else if prev_slot.len() + self.0.len() > u16::MAX.into() {
            Err(io::Error::from(io::ErrorKind::Other))
        } else {
            for i in 0..prev_slot.len() {
                self.0.push(Joint {
                    prev_slot: prev_slot[i],
                    jm: jm[i],
                    ibm: ibm[i],
                    name: name[i].to_string(),
                });
            }
            Ok(self)
        }
    }

    /// Creates the skin.
    ///
    /// This method consumes every pushed joint to create
    /// the skin. The order which a given joint was pushed
    /// identifies its slot in the [`Skin`].
    ///
    /// Fails if no joint has been pushed yet.
    pub fn create(&mut self) -> io::Result<Skin> {
        if !self.0.is_empty() {
            Ok(Skin(mem::take(&mut self.0)))
        } else {
            Err(io::Error::from(io::ErrorKind::InvalidInput))
        }
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}
