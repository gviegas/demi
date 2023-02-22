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

struct NodeData {
    data: Node,
    world: Mat4<f32>,
    changed: bool,
    hidden: bool,
    node: usize,
}

#[derive(Copy, Clone)]
pub struct NodeId(usize);

pub struct NodeIdRemap {
    pub old_id: NodeId,
    pub new_id: NodeId,
}

pub struct Subgraph {
    nodes: Vec<(NodeLink, NodeId)>,
    data: Vec<NodeData>,
}

pub struct Graph {
    nodes: Vec<NodeLink>,
    nbits: BitVec<u32>,
    data: Vec<NodeData>,
}

const NBITS_GRAN: usize = u32::BITS as _;

impl Graph {
    /// Creates an empty graph.
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            nbits: BitVec::new(),
            data: vec![],
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

        self.nodes[idx].data = self.data.len();
        self.nodes[idx].typ = match node {
            Node::Drawable(..) => NodeType::Drawable,
            Node::Light(..) => NodeType::Light,
            Node::Xform(..) => NodeType::Xform,
        };
        self.data.push(NodeData {
            data: node,
            world: Default::default(),
            changed: true,
            hidden: false,
            node: idx,
        });

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
            let swap = g.data.last().unwrap().node;
            g.nodes[swap].data = n.data;
            g.data.swap_remove(n.data).data
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
        let data = self.nodes[node.0].data;
        match &self.data[data].data {
            Node::Drawable(d, _) => Some(d),
            _ => None,
        }
    }

    /// Returns a reference to the [`Light`] that a given
    /// [`NodeId`] identifies, or `None` if it is not a
    /// [`Node::Light`].
    pub fn light(&self, node: NodeId) -> Option<&Light> {
        let data = self.nodes[node.0].data;
        match &self.data[data].data {
            Node::Light(l, _) => Some(l),
            _ => None,
        }
    }

    /// Returns a reference to the local transform that a
    /// given [`NodeId`] identifies.
    pub fn local(&self, node: NodeId) -> &Mat4<f32> {
        let data = self.nodes[node.0].data;
        match &self.data[data].data {
            Node::Drawable(_, x) => x,
            Node::Light(_, x) => x,
            Node::Xform(x) => x,
        }
    }

    /// Returns a mutable reference to the local transform
    /// that a given [`NodeId`] identifies.
    ///
    /// The sub-graph rooted at `node` becomes out of date.
    pub fn local_mut(&mut self, node: NodeId) -> &mut Mat4<f32> {
        let data = self.nodes[node.0].data;
        self.data[data].changed = true;
        match &mut self.data[data].data {
            Node::Drawable(_, x) => x,
            Node::Light(_, x) => x,
            Node::Xform(x) => x,
        }
    }

    /// Returns a reference to the world transform that a
    /// given [`NodeId`] identifies.
    ///
    /// This transform is not necessarily up to date.
    pub fn world(&self, node: NodeId) -> &Mat4<f32> {
        let data = self.nodes[node.0].data;
        &self.data[data].world
    }

    /// Returns the length of the graph.
    pub fn len(&self) -> usize {
        self.data.len()
    }
}
