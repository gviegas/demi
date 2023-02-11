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
#[derive(Copy, Clone, Debug)]
pub struct NodeId {
    node_type: NodeType,
    node_idx: usize,
}

/// Type of a `Node`.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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
    // `nodes` contains indices into `NodeData` vectors.
    nodes: Vec<Option<usize>>,
    node_idx: usize,
    none_cnt: usize,
    drawables: Vec<NodeData<Drawable>>,
    lights: Vec<NodeData<Light>>,
    xforms: Vec<NodeData<()>>,
    // TODO...
}

impl Scene {
    /// Creates an empty scene.
    pub fn new() -> Self {
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

    /// Inserts a new node.
    ///
    /// NOTE: The `NodeId` returned by this method must not be used
    /// with `Scene`s other than the one that produced it.
    pub fn insert(&mut self, node: Node, prev: Option<NodeId>) -> NodeId {
        // If `prev` is not provided, we insert the new node into the
        // graph's root transform. Each of these nodes can then be
        // treated as a separate graph.
        let root = self.graph.id();
        let prev = if let Some(x) = prev {
            let prev_idx = self.nodes[x.node_idx].unwrap();
            match x.node_type {
                NodeType::Drawable => self.drawables[prev_idx].xform_id,
                NodeType::Light => self.lights[prev_idx].xform_id,
                NodeType::Xform => self.xforms[prev_idx].xform_id,
            }
        } else {
            self.graph.id()
        };
        let node_idx = if self.none_cnt > 0 {
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
                self.nodes[node_idx] = Some(self.drawables.len());
                self.drawables.push(NodeData {
                    data: d,
                    xform_id: self.graph.insert(prev, x),
                    node_idx,
                });
                NodeType::Drawable
            }
            Node::Light(l, x) => {
                self.nodes[node_idx] = Some(self.lights.len());
                self.lights.push(NodeData {
                    data: l,
                    xform_id: self.graph.insert(prev, x),
                    node_idx,
                });
                NodeType::Light
            }
            Node::Xform(x) => {
                self.nodes[node_idx] = Some(self.xforms.len());
                self.xforms.push(NodeData {
                    data: (),
                    xform_id: self.graph.insert(prev, x),
                    node_idx,
                });
                NodeType::Xform
            }
        };
        NodeId {
            node_type,
            node_idx,
        }
    }

    /// Removes a given node.
    ///
    /// NOTE: `node_id` must have been produced by a call to `self.insert`.
    pub fn remove(&mut self, node_id: NodeId) -> Node {
        // TODO: Need a quick way to check whether a node is an orphan,
        // so it can be ignored during rendering (as expected).
        // Either do it here or in the `Transform`.
        let data_idx = self.nodes[node_id.node_idx].take().unwrap();
        self.node_idx = node_id.node_idx;
        self.none_cnt += 1;
        match node_id.node_type {
            NodeType::Drawable => {
                let swap = self.drawables.last().unwrap().node_idx;
                let drawable = if swap != node_id.node_idx {
                    self.nodes[swap] = Some(data_idx);
                    self.drawables.swap_remove(data_idx)
                } else {
                    self.drawables.pop().unwrap()
                };
                let xform = self.graph.remove(drawable.xform_id);
                Node::Drawable(drawable.data, xform)
            }
            NodeType::Light => {
                let swap = self.lights.last().unwrap().node_idx;
                let light = if swap != node_id.node_idx {
                    self.nodes[swap] = Some(data_idx);
                    self.lights.swap_remove(data_idx)
                } else {
                    self.lights.pop().unwrap()
                };
                let xform = self.graph.remove(light.xform_id);
                Node::Light(light.data, xform)
            }
            NodeType::Xform => {
                let swap = self.xforms.last().unwrap().node_idx;
                let xform = if swap != node_id.node_idx {
                    self.nodes[swap] = Some(data_idx);
                    self.xforms.swap_remove(data_idx)
                } else {
                    self.xforms.pop().unwrap()
                };
                let xform = self.graph.remove(xform.xform_id);
                Node::Xform(xform)
            }
        }
    }

    /// Returns a reference to a node's `Drawable`.
    ///
    /// Panics if `node_id` was generated by something other than a
    /// `Node::Drawable`'s insertion.
    pub fn drawable(&self, node_id: NodeId) -> &Drawable {
        match node_id.node_type {
            NodeType::Drawable => {
                let data_idx = self.nodes[node_id.node_idx].unwrap();
                &self.drawables[data_idx].data
            }
            _ => panic!("Not a Drawable node: {:?}", node_id),
        }
    }

    /// Returns a mutable reference to a node's `Drawable`.
    ///
    /// Panics if `node_id` was generated by something other than a
    /// `Node::Drawable`'s insertion.
    pub fn drawable_mut(&mut self, node_id: NodeId) -> &mut Drawable {
        match node_id.node_type {
            NodeType::Drawable => {
                let data_idx = self.nodes[node_id.node_idx].unwrap();
                &mut self.drawables[data_idx].data
            }
            _ => panic!("Not a Drawable node: {:?}", node_id),
        }
    }

    /// Returns a reference to a node's `Light`.
    ///
    /// Panics if `node_id` was generated by something other than a
    /// `Node::Light`'s insertion.
    pub fn light(&self, node_id: NodeId) -> &Light {
        match node_id.node_type {
            NodeType::Light => {
                let data_idx = self.nodes[node_id.node_idx].unwrap();
                &self.lights[data_idx].data
            }
            _ => panic!("Not a Light node: {:?}", node_id),
        }
    }

    /// Returns a mutable reference to a node's `Light`.
    ///
    /// Panics if `node_id` was generated by something other than a
    /// `Node::Light`'s insertion.
    pub fn light_mut(&mut self, node_id: NodeId) -> &mut Light {
        match node_id.node_type {
            NodeType::Light => {
                let data_idx = self.nodes[node_id.node_idx].unwrap();
                &mut self.lights[data_idx].data
            }
            _ => panic!("Not a Light node: {:?}", node_id),
        }
    }

    /// Returns a reference to a node's local transform.
    ///
    /// This is a shorthand for `scene.graph().local(scene.xform_id(node_id))`.
    ///
    /// Call this method rather than `local_mut` whenever possible.
    pub fn local(&self, node_id: NodeId) -> &Mat4<f32> {
        self.graph.local(self.xform_id(node_id))
    }

    /// Returns a mutable reference to a node's local transform.
    ///
    /// NOTE: See `transform::Transform::local_mut` for usage advice.
    pub fn local_mut(&mut self, node_id: NodeId) -> &mut Mat4<f32> {
        let data_idx = self.nodes[node_id.node_idx].unwrap();
        let xform_id = match node_id.node_type {
            NodeType::Drawable => self.drawables[data_idx].xform_id,
            NodeType::Light => self.lights[data_idx].xform_id,
            NodeType::Xform => self.xforms[data_idx].xform_id,
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

    /// Returns a node's `XformId`.
    pub fn xform_id(&self, node_id: NodeId) -> XformId {
        let data_idx = self.nodes[node_id.node_idx].unwrap();
        match node_id.node_type {
            NodeType::Drawable => self.drawables[data_idx].xform_id,
            NodeType::Light => self.lights[data_idx].xform_id,
            NodeType::Xform => self.xforms[data_idx].xform_id,
        }
    }

    // TODO: Graph update, camera, sky box, ibl...
}

impl Default for Scene {
    /// Creates an empty scene.
    fn default() -> Self {
        Self::new()
    }
}
