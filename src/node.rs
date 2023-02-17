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

    pub fn insert(&mut self, node: Node, prev: Option<NodeId>) -> NodeId {
        todo!();
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
