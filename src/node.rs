// Copyright 2023 Gustavo C. Viegas. All rights reserved.

//! Node graph.

#![allow(unused_variables)] // TODO

use std::mem;

use crate::bit_vec::BitVec;
use crate::drawable::Drawable;
use crate::light::Light;
use crate::linear::Mat4;

#[cfg(test)]
mod tests;

pub enum Node {
    Drawable(Drawable, Mat4<f32>),
    Light(Light, Mat4<f32>),
    Xform(Mat4<f32>),
}

#[derive(Copy, Clone)]
enum NodeType {
    Drawable,
    Light,
    Xform,
}

// Use this sentinel value to identify a link's absence
// since `Option<usize>` has twice the size of `usize`.
const NONE: usize = usize::MAX;

struct NodeLink {
    next: usize,
    prev: usize,
    sub: usize,
    typ: NodeType,
    data: usize,
}

struct NodeData<T> {
    local: Mat4<f32>,
    world: Mat4<f32>,
    changed: bool,
    hidden: bool,
    node: usize,
    data: T,
}

#[derive(Copy, Clone)]
pub struct NodeId(usize);

pub struct NodeIdRemap {
    pub old_id: NodeId,
    pub new_id: NodeId,
}

pub struct Subgraph {
    nodes: Vec<(NodeLink, NodeId)>,
    drawables: Vec<NodeData<Drawable>>,
    lights: Vec<NodeData<Light>>,
    xforms: Vec<NodeData<()>>,
}

pub struct Graph {
    nodes: Vec<NodeLink>,
    nbits: BitVec<u32>,
    drawables: Vec<NodeData<Drawable>>,
    lights: Vec<NodeData<Light>>,
    xforms: Vec<NodeData<()>>,
}

const NBITS_GRAN: usize = u32::BITS as _;

impl Graph {
    /// Creates an empty graph.
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            nbits: BitVec::new(),
            drawables: vec![],
            lights: vec![],
            xforms: vec![],
        }
    }

    /// Inserts `node` as descendant of `prev`.
    /// If `prev` is `None`, then `node` is inserted in the graph
    /// as an unconnected node.
    /// It returns a [`NodeId`] that identifies `node` in this
    /// specific graph.
    pub fn insert(&mut self, node: Node, prev: Option<NodeId>) -> NodeId {
        let idx = self
            .nbits
            .find()
            .or_else(|| {
                let nlen = usize::max(NBITS_GRAN, self.nodes.len() * 2);
                self.nodes.resize_with(nlen, || NodeLink {
                    next: NONE,
                    prev: NONE,
                    sub: NONE,
                    typ: NodeType::Xform,
                    data: NONE,
                });
                let nlen = (self.nodes.len() - self.nbits.len()) / NBITS_GRAN;
                self.nbits.grow(nlen)
            })
            .unwrap();
        self.nbits.set(idx);

        if let Some(NodeId(prev)) = prev {
            match self.nodes[prev].sub {
                NONE => self.nodes[idx].next = NONE,
                sub => {
                    self.nodes[idx].next = sub;
                    self.nodes[sub].prev = idx;
                }
            }
            self.nodes[prev].sub = idx;
            self.nodes[idx].prev = prev;
        } else {
            self.nodes[idx].next = NONE;
            self.nodes[idx].prev = NONE;
        }
        self.nodes[idx].sub = NONE;

        let (typ, data) = match node {
            Node::Drawable(d, x) => {
                self.drawables.push(NodeData {
                    local: x,
                    world: Default::default(),
                    changed: true,
                    hidden: false,
                    node: idx,
                    data: d,
                });
                (NodeType::Drawable, self.drawables.len() - 1)
            }
            Node::Light(l, x) => {
                self.lights.push(NodeData {
                    local: x,
                    world: Default::default(),
                    changed: true,
                    hidden: false,
                    node: idx,
                    data: l,
                });
                (NodeType::Light, self.lights.len() - 1)
            }
            Node::Xform(x) => {
                self.xforms.push(NodeData {
                    local: x,
                    world: Default::default(),
                    changed: true,
                    hidden: false,
                    node: idx,
                    data: (),
                });
                (NodeType::Xform, self.xforms.len() - 1)
            }
        };
        self.nodes[idx].typ = typ;
        self.nodes[idx].data = data;

        NodeId(idx)
    }

    /// Removes `node` and its descendants.
    /// The [`NodeId`] of every removed node becomes invalid and
    /// thus must no longer be used.
    pub fn remove(&mut self, node: NodeId) -> Vec<Node> {
        let idx = node.0;
        let node = remove_link(self, idx);
        self.nbits.unset(idx);
        let mut nodes = vec![remove_node(self, &node)];

        if node.next != NONE {
            self.nodes[node.next].prev = node.prev;
        }
        if node.prev != NONE {
            if self.nodes[node.prev].sub == idx {
                self.nodes[node.prev].sub = node.next;
            } else {
                self.nodes[node.prev].next = node.next;
            }
        }
        if node.sub != NONE {
            let mut desc = vec![node.sub];
            while let Some(idx) = desc.pop() {
                let node = remove_link(self, idx);
                self.nbits.unset(idx);
                nodes.push(remove_node(self, &node));
                if node.next != NONE {
                    desc.push(node.next);
                }
                if node.sub != NONE {
                    desc.push(node.sub);
                }
            }
        }

        return nodes;

        fn remove_link(g: &mut Graph, n: usize) -> NodeLink {
            mem::replace(
                &mut g.nodes[n],
                NodeLink {
                    next: NONE,
                    prev: NONE,
                    sub: NONE,
                    typ: NodeType::Xform,
                    data: NONE,
                },
            )
        }

        fn remove_node(g: &mut Graph, n: &NodeLink) -> Node {
            match n.typ {
                NodeType::Drawable => {
                    let swap = g.drawables.last().unwrap().node;
                    g.nodes[swap].data = n.data;
                    let data = g.drawables.swap_remove(n.data);
                    Node::Drawable(data.data, data.local)
                }
                NodeType::Light => {
                    let swap = g.lights.last().unwrap().node;
                    g.nodes[swap].data = n.data;
                    let data = g.lights.swap_remove(n.data);
                    Node::Light(data.data, data.local)
                }
                NodeType::Xform => {
                    let swap = g.xforms.last().unwrap().node;
                    g.nodes[swap].data = n.data;
                    let data = g.xforms.swap_remove(n.data);
                    Node::Xform(data.local)
                }
            }
        }
    }

    pub fn insert_subgraph(
        &mut self,
        subgraph: Subgraph,
        prev: Option<NodeId>,
    ) -> Vec<NodeIdRemap> {
        todo!();
    }

    pub fn remove_subgraph(&mut self, node: NodeId) -> Subgraph {
        todo!();
    }

    pub fn update(&mut self) {
        todo!();
    }

    /// Returns a reference to the [`Drawable`] that a given
    /// [`NodeId`] identifies, or `None` if it is not a
    /// [`Node::Drawable`].
    pub fn drawable(&self, node: NodeId) -> Option<&Drawable> {
        match self.nodes[node.0].typ {
            NodeType::Drawable => {
                let data = self.nodes[node.0].data;
                Some(&self.drawables[data].data)
            }
            _ => None,
        }
    }

    /// Returns a reference to the [`Light`] that a given
    /// [`NodeId`] identifies, or `None` if it is not a
    /// [`Node::Light`].
    pub fn light(&self, node: NodeId) -> Option<&Light> {
        match self.nodes[node.0].typ {
            NodeType::Light => {
                let data = self.nodes[node.0].data;
                Some(&self.lights[data].data)
            }
            _ => None,
        }
    }

    /// Returns a reference to the local transform that a
    /// given [`NodeId`] identifies.
    pub fn local(&self, node: NodeId) -> &Mat4<f32> {
        let data = self.nodes[node.0].data;
        match self.nodes[node.0].typ {
            NodeType::Drawable => &self.drawables[data].local,
            NodeType::Light => &self.lights[data].local,
            NodeType::Xform => &self.xforms[data].local,
        }
    }

    /// Returns a mutable reference to the local transform
    /// that a given [`NodeId`] identifies.
    ///
    /// The sub-graph rooted at `node` becomes out of date.
    pub fn local_mut(&mut self, node: NodeId) -> &mut Mat4<f32> {
        let data = self.nodes[node.0].data;
        match self.nodes[node.0].typ {
            NodeType::Drawable => {
                self.drawables[data].changed = true;
                &mut self.drawables[data].local
            }
            NodeType::Light => {
                self.lights[data].changed = true;
                &mut self.lights[data].local
            }
            NodeType::Xform => {
                self.xforms[data].changed = true;
                &mut self.xforms[data].local
            }
        }
    }

    /// Returns a reference to the world transform that a
    /// given [`NodeId`] identifies.
    ///
    /// This transform is not necessarily up to date.
    pub fn world(&self, node: NodeId) -> &Mat4<f32> {
        let data = self.nodes[node.0].data;
        match self.nodes[node.0].typ {
            NodeType::Drawable => &self.drawables[data].world,
            NodeType::Light => &self.lights[data].world,
            NodeType::Xform => &self.xforms[data].world,
        }
    }

    /// Returns the length of the graph.
    pub fn len(&self) -> usize {
        self.drawables.len() + self.lights.len() + self.xforms.len()
    }

    /// Returns the number of [`Node::Drawable`]s that the
    /// graph contains.
    pub fn drawable_len(&self) -> usize {
        self.drawables.len()
    }

    /// Returns the number of [`Node::Light`]s that the
    /// graph contains.
    pub fn light_len(&self) -> usize {
        self.lights.len()
    }

    /// Returns the number of [`Node::Xform`]s that the
    /// graph contains.
    pub fn xform_len(&self) -> usize {
        self.xforms.len()
    }
}
