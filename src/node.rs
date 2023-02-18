// Copyright 2023 Gustavo C. Viegas. All rights reserved.

//! Node graph.

#![allow(unused_variables)] // TODO

use crate::bit_vec::BitVec;
use crate::drawable::Drawable;
use crate::light::Light;
use crate::linear::Mat4;

pub enum Node {
    Drawable(Drawable, Mat4<f32>),
    Light(Light, Mat4<f32>),
    Xform(Mat4<f32>),
}

enum NodeType {
    Drawable,
    Light,
    Xform,
}

struct NodeLink {
    next: Option<usize>,
    prev: Option<usize>,
    sub: Option<usize>,
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

pub struct NodeId {
    typ: NodeType,
    node: usize,
}

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
    pub fn new(nodes: Vec<Node>) -> Self {
        if nodes.is_empty() {
            Self {
                nodes: vec![],
                nbits: BitVec::new(),
                drawables: vec![],
                lights: vec![],
                xforms: vec![],
            }
        } else {
            todo!();
        }
    }

    /// Inserts `node` as descendant of `prev`.
    /// If `prev` is `None`, then `node` is inserted in the graph
    /// unconnected.
    /// It returns a [`NodeId`] that identifies `node` in this
    /// specific graph.
    pub fn insert(&mut self, node: Node, prev: Option<NodeId>) -> NodeId {
        let idx = self
            .nbits
            .find()
            .or_else(|| {
                self.nodes
                    .resize_with(self.nodes.len() + NBITS_GRAN, || NodeLink {
                        next: None,
                        prev: None,
                        sub: None,
                        data: usize::MAX,
                    });
                self.nbits.grow(1)
            })
            .unwrap();
        self.nbits.set(idx);

        if let Some(NodeId { node: prev, .. }) = prev {
            if let Some(sub) = self.nodes[prev].sub {
                self.nodes[idx].next = Some(sub);
                self.nodes[sub].prev = Some(idx);
            } else {
                self.nodes[idx].next = None;
            }
            self.nodes[prev].sub = Some(idx);
            self.nodes[idx].prev = Some(prev);
        } else {
            self.nodes[idx].next = None;
            self.nodes[idx].prev = None;
        }
        self.nodes[idx].sub = None;

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
        self.nodes[idx].data = data;

        NodeId { typ, node: idx }
    }

    pub fn remove(&mut self, node: NodeId) -> Node {
        todo!();
    }

    pub fn merge(&mut self, subgraph: Subgraph, prev: Option<NodeId>) -> Vec<NodeIdRemap> {
        todo!();
    }

    pub fn split(&mut self, node: NodeId) -> Subgraph {
        todo!();
    }

    pub fn update(&mut self) {
        todo!();
    }

    // TODO: Getters/setters.
}
