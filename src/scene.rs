// Copyright 2022 Gustavo C. Viegas. All rights reserved.

//! Scene graph.

#[cfg(test)]
mod tests;

use crate::drawable::Drawable;
use crate::light::Light;
use crate::linear::Mat4;
use crate::transform::{Transform, XformId};

/// Node data.
#[derive(Debug)]
struct NodeData<T> {
    data: T,
    xform_id: XformId,
    node_idx: usize,
    // TODO: Flags.
}

/// Identifier of a `Node`.
#[derive(Debug)]
pub struct NodeId {
    node_type: NodeType,
    index: usize,
}

/// Type of a `Node`.
#[derive(Eq, PartialEq, Debug)]
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
    drawables: Vec<NodeData<Drawable>>,
    lights: Vec<NodeData<Light>>,
    xforms: Vec<NodeData<()>>,
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
    pub fn insert(&mut self, prev: Option<&NodeId>, node: Node) -> NodeId {
        // If `prev` is not provided, we insert the new node into the
        // graph's root transform. Each of these nodes can then be
        // treated as a separate graph.
        let root = self.graph.id();
        let prev = if let Some(x) = prev {
            let prev_idx = self.nodes[x.index].unwrap();
            match x.node_type {
                NodeType::Drawable => &self.drawables[prev_idx].xform_id,
                NodeType::Light => &self.lights[prev_idx].xform_id,
                NodeType::Xform => &self.xforms[prev_idx].xform_id,
            }
        } else {
            &root
        };
        let index = if self.none_cnt > 0 {
            // There is a vacant node that we can use.
            let n = self.nodes.len();
            let mut i = self.node_idx;
            while self.nodes[i].is_some() {
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
            Node::Drawable(d, x) => {
                self.nodes[index] = Some(self.drawables.len());
                self.drawables.push(NodeData {
                    data: d,
                    xform_id: self.graph.insert(prev, x),
                    node_idx: index,
                });
                NodeType::Drawable
            }
            Node::Light(l, x) => {
                self.nodes[index] = Some(self.lights.len());
                self.lights.push(NodeData {
                    data: l,
                    xform_id: self.graph.insert(prev, x),
                    node_idx: index,
                });
                NodeType::Light
            }
            Node::Xform(x) => {
                self.nodes[index] = Some(self.xforms.len());
                self.xforms.push(NodeData {
                    data: (),
                    xform_id: self.graph.insert(prev, x),
                    node_idx: index,
                });
                NodeType::Xform
            }
        };
        NodeId { node_type, index }
    }

    /// Removes a given node.
    ///
    /// NOTE: `node_id` must have been produced by a call to `Scene::insert`.
    pub fn remove(&mut self, node_id: NodeId) -> Node {
        // TODO: Need a quick way to check whether a node is an orphan,
        // so it can be ignored during rendering (as expected).
        // Either do it here or in the `Transform`.
        let index = self.nodes[node_id.index].take().unwrap();
        self.node_idx = node_id.index;
        self.none_cnt += 1;
        match node_id.node_type {
            NodeType::Drawable => {
                let swap = self.drawables.last().unwrap().node_idx;
                let drawable = if swap != node_id.index {
                    self.nodes[swap] = Some(index);
                    self.drawables.swap_remove(index)
                } else {
                    self.drawables.pop().unwrap()
                };
                let xform = self.graph.remove(drawable.xform_id);
                Node::Drawable(drawable.data, xform)
            }
            NodeType::Light => {
                let swap = self.lights.last().unwrap().node_idx;
                let light = if swap != node_id.index {
                    self.nodes[swap] = Some(index);
                    self.lights.swap_remove(index)
                } else {
                    self.lights.pop().unwrap()
                };
                let xform = self.graph.remove(light.xform_id);
                Node::Light(light.data, xform)
            }
            NodeType::Xform => {
                let swap = self.xforms.last().unwrap().node_idx;
                let xform = if swap != node_id.index {
                    self.nodes[swap] = Some(index);
                    self.xforms.swap_remove(index)
                } else {
                    self.xforms.pop().unwrap()
                };
                let xform = self.graph.remove(xform.xform_id);
                Node::Xform(xform)
            }
        }
    }

    /// Returns a mutable reference to a node's local transform.
    ///
    /// NOTE: See `transform::Transform::local_mut` for usage.
    pub fn local_mut(&mut self, node_id: &NodeId) -> &mut Mat4<f32> {
        let index = self.nodes[node_id.index].unwrap();
        let xform_id = match node_id.node_type {
            NodeType::Drawable => &self.drawables[index].xform_id,
            NodeType::Light => &self.lights[index].xform_id,
            NodeType::Xform => &self.xforms[index].xform_id,
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
            NodeType::Drawable => &self.drawables[index].xform_id,
            NodeType::Light => &self.lights[index].xform_id,
            NodeType::Xform => &self.xforms[index].xform_id,
        }
    }

    // TODO: Graph update, camera, sky box, ibl...
}
