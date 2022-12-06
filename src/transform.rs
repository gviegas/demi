// Copyright 2022 Gustavo C. Viegas. All rights reserved.

#![allow(dead_code)] // TODO
#![allow(unused_variables)] // TODO

use crate::linear::Mat4;

/// Identifier of a transform.
#[derive(Debug)]
pub struct XformId(usize);

/// Node in a transform graph.
#[derive(Debug)]
struct XformNode {
    prev: Option<usize>,
    next: Option<usize>,
    sub: Option<usize>,
    data: usize,
}

/// Data of a transform.
#[derive(Debug)]
struct XformData {
    // TODO: Consider storing the local transform
    // as TRS properties instead.
    local: Mat4<f32>,
    world: Mat4<f32>,
    world_inv: Mat4<f32>,
}

/// Transform.
#[derive(Debug)]
pub struct Transform {
    // TODO: Node management.
    nodes: Vec<Option<XformNode>>,
    data: Vec<XformData>,
}

impl Transform {
    /// Creates a new root transform.
    pub fn new(xform: &Mat4<f32>) -> Self {
        Self {
            nodes: vec![Some(XformNode {
                prev: None,
                next: None,
                sub: None,
                data: 0,
            })],
            data: vec![XformData {
                local: xform.clone(),
                world: xform.clone(),
                world_inv: xform.invert(),
            }],
        }
    }

    /// Returns the root transform's identifier.
    pub fn id(&self) -> XformId {
        XformId(0)
    }

    /// Returns the length of the transform graph.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Inserts a new transform.
    pub fn insert(&mut self, prev: &XformId, xform: &Mat4<f32>) -> XformId {
        todo!();
    }

    /// Removes a given transform.
    pub fn remove(&mut self, id: XformId) {
        todo!();
    }

    /// Returns a reference to a given local transform.
    pub fn local(&self, id: &XformId) -> &Mat4<f32> {
        // TODO: Validate.
        let data_idx = self.nodes[id.0].as_ref().unwrap().data;
        &self.data[data_idx].local
    }

    /// Returns a mutable reference to a given local transform.
    pub fn local_mut(&mut self, id: &XformId) -> &mut Mat4<f32> {
        // TODO: Validate.
        let data_idx = self.nodes[id.0].as_ref().unwrap().data;
        &mut self.data[data_idx].local
    }

    /// Returns a reference to a given world transform.
    pub fn world(&self, id: &XformId) -> &Mat4<f32> {
        // TODO: Validate.
        let data_idx = self.nodes[id.0].as_ref().unwrap().data;
        &self.data[data_idx].world
    }
}
