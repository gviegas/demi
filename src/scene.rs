// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use std::mem::{self, Discriminant};

use crate::drawable::Drawable;
use crate::light::Light;
use crate::linear::Mat4;
use crate::transform::{Transform, XformId};

/// Identifier of a `Node`.
#[derive(Debug)]
pub struct NodeId(Discriminant<Node>, usize);

impl NodeId {
    /// Returns the discriminant of the `Node`.
    pub fn discriminant(&self) -> Discriminant<Node> {
        self.0
    }

    /// Returns a reference to the node's `XformId`.
    pub fn xform_id(&self) -> &XformId {
        todo!();
    }
}

/// Node in a `Scene` graph.
#[derive(Debug)]
pub enum Node {
    Drawable(Drawable),
    Light(Light),
    Xform(Mat4<f32>),
}

/// Scene.
#[allow(dead_code)] // TODO
#[derive(Debug)]
pub struct Scene {
    graph: Transform,
    nodes: Vec<Option<NodeId>>,
    node_idx: usize,
    node_cnt: usize,
    drawables: Vec<Drawable>,
    lights: Vec<Light>,
    // TODO...
}

impl Default for Scene {
    /// Creates an empty scene.
    fn default() -> Self {
        Self {
            graph: Transform::default(),
            nodes: vec![],
            node_idx: 0,
            node_cnt: 0,
            drawables: vec![],
            lights: vec![],
        }
    }
}

impl Scene {
    /// Inserts a new node.
    ///
    /// NOTE: The `NodeId` returned by this method must not be used
    /// with `Scene`s other than the one that produced it.
    #[allow(unused_variables)] // TODO
    pub fn insert(&mut self, prev: Option<&NodeId>, node: Node) -> NodeId {
        todo!();
    }

    /// Removes a given node.
    ///
    /// NOTE: `id` must have been produced by a call to `Scene::insert`.
    #[allow(unused_variables)] // TODO
    pub fn remove(&mut self, id: NodeId) -> Node {
        todo!();
    }

    /// Returns a mutable reference to a `Node`'s local transform.
    ///
    /// NOTE: See `transform::Transform::local_mut`.
    pub fn local_mut(&mut self, id: &NodeId) -> &mut Mat4<f32> {
        self.graph.local_mut(id.xform_id())
    }

    /// Returns a reference to the `Scenes`'s `Transform` (i.e., the scene graph).
    ///
    /// The scene is responsible for managing the transform graph, thus mutable
    /// access is not provided. One can use `insert`, `remove` and `local_mut`
    /// to mutate the graph - the `Scene` will forward these calls to the
    /// `Transform`'s methods of same name.
    ///
    /// Directly updating the world is not possible, and neither is changing the
    /// root's transform.
    pub fn graph(&self) -> &Transform {
        &self.graph
    }

    // TODO: Graph update, camera, sky box, ibl...
}
