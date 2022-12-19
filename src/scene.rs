// Copyright 2022 Gustavo C. Viegas. All rights reserved.

//! Scene graph.

#[cfg(test)]
mod tests;

use crate::drawable::Drawable;
use crate::light::Light;
use crate::linear::Mat4;
use crate::transform::{Transform, XformId};

/// Identifier of a `Node`.
#[derive(Debug)]
pub struct NodeId {
    node_type: NodeType,
    index: usize,
}

/// Type of a `Node`.
#[derive(Debug)]
enum NodeType {
    Drawable,
    Light,
    Xform,
}

/// Node in a `Scene` graph.
#[derive(Debug)]
pub enum Node {
    Drawable(Drawable, Mat4<f32>),
    Light(Light, Mat4<f32>),
    Xform(Mat4<f32>),
}

/// Scene.
#[allow(dead_code)] // TODO
#[derive(Debug)]
pub struct Scene {
    graph: Transform,
    // Entries in `nodes` are indices in `drawables`, `lights`
    // or `xforms`. `NodeId.node_type` is used to identify the
    // vector to index into.
    nodes: Vec<Option<usize>>,
    node_idx: usize,
    none_cnt: usize,
    // The `usize` here (in the case of `drawables` and `lights`,
    // stored in the structs) is a back-reference into `nodes`.
    drawables: Vec<Drawable>,
    lights: Vec<Light>,
    xforms: Vec<(XformId, usize)>,
    // TODO...
}

impl Default for Scene {
    /// Creates an empty scene.
    fn default() -> Self {
        Self {
            graph: Transform::default(),
            nodes: vec![],
            node_idx: 0,
            none_cnt: 0,
            drawables: vec![],
            lights: vec![],
            xforms: vec![],
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
        // If `prev` is not provided, insert the new node into the
        // graph's root transform. Each of these nodes can then be
        // treated as a separate graph.
        let root = self.graph.id();
        let prev = if let Some(x) = prev {
            let prev_idx = self.nodes[x.index].unwrap();
            match x.node_type {
                NodeType::Drawable => &self.drawables[prev_idx].node.as_ref().unwrap().0,
                NodeType::Light => &self.lights[prev_idx].node.as_ref().unwrap().0,
                NodeType::Xform => &self.xforms[prev_idx].0,
            }
        } else {
            &root
        };
        let index = if self.none_cnt > 0 {
            // There is a vacant node that we can use.
            let n = self.nodes.len();
            let mut i = self.node_idx;
            while self.nodes[i].is_none() {
                i = (i + 1) % n;
            }
            self.node_idx = n / 2;
            self.none_cnt -= 1;
            i
        } else {
            // No vacant nodes, so push a new one.
            let n = self.nodes.len();
            self.nodes.push(None);
            n
        };
        let node_type = match node {
            Node::Drawable(mut d, x) => {
                debug_assert!(d.node.is_none());
                d.node = Some((self.graph.insert(prev, &x), index));
                self.nodes[index] = Some(self.drawables.len());
                self.drawables.push(d);
                NodeType::Drawable
            }
            Node::Light(mut l, x) => {
                debug_assert!(l.node.is_none());
                l.node = Some((self.graph.insert(prev, &x), index));
                self.nodes[index] = Some(self.lights.len());
                self.lights.push(l);
                NodeType::Light
            }
            Node::Xform(x) => {
                self.nodes[index] = Some(self.xforms.len());
                self.xforms.push((self.graph.insert(prev, &x), index));
                NodeType::Xform
            }
        };
        NodeId { node_type, index }
    }

    /// Removes a given node.
    ///
    /// NOTE: `node_id` must have been produced by a call to `Scene::insert`.
    #[allow(unused_variables)] // TODO
    pub fn remove(&mut self, node_id: NodeId) -> Node {
        todo!();
    }

    /// Returns a mutable reference to a node's local transform.
    ///
    /// NOTE: See `transform::Transform::local_mut`.
    pub fn local_mut(&mut self, node_id: &NodeId) -> &mut Mat4<f32> {
        let index = self.nodes[node_id.index].unwrap();
        let xform_id = match node_id.node_type {
            NodeType::Drawable => &self.drawables[index].node.as_ref().unwrap().0,
            NodeType::Light => &self.lights[index].node.as_ref().unwrap().0,
            NodeType::Xform => &self.xforms[index].0,
        };
        self.graph.local_mut(xform_id)
    }

    /// Returns a reference to the scene's `Transform` (i.e., the scene graph).
    ///
    /// The scene is responsible for managing the transform graph, thus mutable
    /// access is not provided. One can use `insert`, `remove` and `local_mut`
    /// to mutate the graph - the `Scene` will forward these calls to the
    /// `Transform`'s methods of the same name.
    ///
    /// Directly updating the world is not possible, and neither is changing
    /// the root's transform.
    pub fn graph(&self) -> &Transform {
        &self.graph
    }

    /// Returns a reference to a node's `XformId`.
    ///
    /// Mutable access is not provided. To update the local transform, call
    /// `Scene::local_mut` passing the `NodeId` itself.
    pub fn xform_id(&self, node_id: &NodeId) -> &XformId {
        let index = self.nodes[node_id.index].unwrap();
        match node_id.node_type {
            NodeType::Drawable => &self.drawables[index].node.as_ref().unwrap().0,
            NodeType::Light => &self.lights[index].node.as_ref().unwrap().0,
            NodeType::Xform => &self.xforms[index].0,
        }
    }

    // TODO: Graph update, camera, sky box, ibl...
}
