// Copyright 2023 Gustavo C. Viegas. All rights reserved.

//! Node graph.

#![allow(unused_variables)] // TODO

use std::mem;

use crate::bit_vec::BitVec;
use crate::drawable::Drawable;
use crate::light::Light;
use crate::linear::Mat4;

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
    supr: usize,
    infr: usize,
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

    /// Inserts `node` as descendant of `parent`.
    /// If `parent` is `None`, then `node` is inserted in the graph
    /// as an unconnected node.
    /// It returns a [`NodeId`] that identifies `node` in this
    /// specific graph.
    pub fn insert(&mut self, node: Node, parent: Option<NodeId>) -> NodeId {
        let idx = self
            .nbits
            .find()
            .or_else(|| {
                self.nodes
                    .resize_with(self.nodes.len() + NBITS_GRAN, || NodeLink {
                        next: NONE,
                        prev: NONE,
                        supr: NONE,
                        infr: NONE,
                        typ: NodeType::Xform,
                        data: NONE,
                    });
                self.nbits.grow(1)
            })
            .unwrap();
        self.nbits.set(idx);

        if let Some(NodeId(supr)) = parent {
            match self.nodes[supr].infr {
                NONE => self.nodes[idx].next = NONE,
                infr => {
                    self.nodes[idx].next = infr;
                    self.nodes[infr].prev = idx;
                }
            }
            self.nodes[supr].infr = idx;
            self.nodes[idx].supr = supr;
        } else {
            self.nodes[idx].next = NONE;
            self.nodes[idx].prev = NONE;
            self.nodes[idx].supr = NONE;
        }
        self.nodes[idx].infr = NONE;

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

    /// Removes `node` from the graph.
    /// Descendants of `node` are inherited by its parent, unless
    /// `node` is a root node, in which case its immediate
    /// descendants become roots in the graph.
    pub fn remove(&mut self, node: NodeId) -> Node {
        self.nbits.unset(node.0);
        let idx = node.0;
        let node = mem::replace(
            &mut self.nodes[node.0],
            NodeLink {
                next: NONE,
                prev: NONE,
                supr: NONE,
                infr: NONE,
                typ: NodeType::Xform,
                data: NONE,
            },
        );

        if node.infr != NONE {
            let mut ninfr = node.infr;
            loop {
                self.nodes[ninfr].supr = node.supr;
                let next = self.nodes[ninfr].next;
                if next == NONE {
                    break;
                } else {
                    ninfr = next;
                }
            }
            if node.supr != NONE {
                let sinfr = self.nodes[node.supr].infr;
                if sinfr == idx {
                    self.nodes[node.supr].infr = node.infr;
                    self.nodes[ninfr].next = node.next;
                } else {
                    self.nodes[node.prev].next = node.infr;
                    self.nodes[node.infr].prev = node.prev;
                    self.nodes[ninfr].next = node.next;
                }
                if node.next != NONE {
                    self.nodes[node.next].prev = ninfr;
                }
            }
        } else {
            if node.prev != NONE {
                self.nodes[node.prev].next = node.next;
            } else if node.supr != NONE {
                self.nodes[node.supr].infr = node.next;
            }
            if node.next != NONE {
                self.nodes[node.next].prev = node.prev;
            }
        }

        match node.typ {
            NodeType::Drawable => {
                let swap = self.drawables.last().unwrap().node;
                self.nodes[swap].data = node.data;
                let data = self.drawables.swap_remove(node.data);
                Node::Drawable(data.data, data.local)
            }
            NodeType::Light => {
                let swap = self.lights.last().unwrap().node;
                self.nodes[swap].data = node.data;
                let data = self.lights.swap_remove(node.data);
                Node::Light(data.data, data.local)
            }
            NodeType::Xform => {
                let swap = self.xforms.last().unwrap().node;
                self.nodes[swap].data = node.data;
                let data = self.xforms.swap_remove(node.data);
                Node::Xform(data.local)
            }
        }
    }

    pub fn merge(&mut self, subgraph: Subgraph, parent: Option<NodeId>) -> Vec<NodeIdRemap> {
        todo!();
    }

    pub fn split(&mut self, node: NodeId) -> Subgraph {
        todo!();
    }

    pub fn update(&mut self) {
        todo!();
    }

    /// Returns the parent of `node`, or `None` if `node`
    /// is a root node.
    pub fn parent(&self, node: NodeId) -> Option<NodeId> {
        match self.nodes[node.0].supr {
            NONE => None,
            supr => Some(NodeId(supr)),
        }
    }

    /// Returns the children of `node`, or an empty vector
    /// if `node` is a leaf node.
    pub fn children(&self, node: NodeId) -> Vec<NodeId> {
        match self.nodes[node.0].infr {
            NONE => vec![],
            infr => {
                let mut chdn = vec![NodeId(infr)];
                let mut next = self.nodes[infr].next;
                while next != NONE {
                    chdn.push(NodeId(next));
                    next = self.nodes[next].next;
                }
                chdn
            }
        }
    }

    // TODO: Getters/setters.
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::light::LightType;
    use crate::linear::Vec3;

    impl Graph {
        fn assert(&self) {
            assert_eq!(self.nodes.len(), self.nbits.len());
            assert_eq!(
                self.drawables.len() + self.lights.len() + self.xforms.len(),
                self.nbits.len() - self.nbits.rem()
            );
            for (i, x) in self.nodes.iter().enumerate() {
                assert_ne!(i, x.next);
                assert_ne!(i, x.prev);
                assert_ne!(i, x.supr);
                assert_ne!(i, x.infr);
                assert!(x.next == NONE || x.next < self.nodes.len());
                assert!(x.prev == NONE || x.prev < self.nodes.len());
                assert!(x.supr == NONE || x.supr < self.nodes.len());
                assert!(x.infr == NONE || x.infr < self.nodes.len());
                if self.nbits.is_set(i) {
                    assert!(
                        x.data != NONE
                            && (x.data < self.drawables.len()
                                || x.data < self.lights.len()
                                || x.data < self.xforms.len())
                    );
                }
            }
        }

        fn assert_len(&self, nodes: usize, drawables: usize, lights: usize, xforms: usize) {
            assert_eq!(nodes, self.nodes.len());
            assert_eq!(drawables, self.drawables.len());
            assert_eq!(lights, self.lights.len());
            assert_eq!(xforms, self.xforms.len());
        }

        fn assert_loc(&self, node: NodeId, local: Mat4<f32>) {
            let typ = self.nodes[node.0].typ;
            let data = self.nodes[node.0].data;
            let m = match typ {
                NodeType::Drawable => {
                    assert_eq!(self.drawables[data].node, node.0);
                    self.drawables[data].local
                }
                NodeType::Light => {
                    assert_eq!(self.lights[data].node, node.0);
                    self.lights[data].local
                }
                NodeType::Xform => {
                    assert_eq!(self.xforms[data].node, node.0);
                    self.xforms[data].local
                }
            };
            assert_eq!(local, m);
        }

        fn assert_unconn(&self, node: NodeId) {
            let &NodeLink {
                next,
                prev,
                supr,
                infr,
                typ,
                data,
            } = &self.nodes[node.0];
            assert_eq!(next, NONE);
            assert_eq!(prev, NONE);
            assert_eq!(supr, NONE);
            assert_eq!(infr, NONE);
            assert!(
                data != NONE
                    && data
                        < match typ {
                            NodeType::Drawable => self.drawables.len(),
                            NodeType::Light => self.lights.len(),
                            NodeType::Xform => self.xforms.len(),
                        }
            );
        }

        fn assert_hier(&self, node: NodeId, parent: Option<NodeId>, mut children: Vec<NodeId>) {
            assert_eq!(
                parent.map_or(NONE, |x| x.0),
                self.parent(node).map_or(NONE, |x| x.0)
            );

            let mut other = self.children(node);
            assert_eq!(children.len(), other.len());
            children.sort_unstable_by(|a, b| a.0.cmp(&b.0));
            other.sort_unstable_by(|a, b| a.0.cmp(&b.0));
            children
                .into_iter()
                .zip(other)
                .for_each(|(a, b)| assert_eq!(a.0, b.0));
        }
    }

    #[test]
    fn insert_one() {
        // TODO:  Node::Drawable.

        let mut g = Graph::new();
        let n = g.insert(
            Node::Light(
                Light::new_white(LightType::Directional, 500.0),
                Mat4::from(1.5),
            ),
            None,
        );
        g.assert();
        g.assert_len(NBITS_GRAN, 0, 1, 0);
        g.assert_loc(n, Mat4::from(1.5));
        g.assert_unconn(n);
        g.assert_hier(n, None, vec![]);

        let mut g = Graph::new();
        let n = g.insert(Node::Xform(Mat4::from(-1.0)), None);
        g.assert();
        g.assert_len(NBITS_GRAN, 0, 0, 1);
        g.assert_loc(n, Mat4::from(-1.0));
        g.assert_unconn(n);
        g.assert_hier(n, None, vec![]);
    }

    #[test]
    fn insert() {
        let mut g = Graph::new();
        g.assert();
        g.assert_len(0, 0, 0, 0);

        let n1 = g.insert(
            Node::Light(
                Light::new_white(LightType::Directional, 500.0),
                Mat4::from(1.0),
            ),
            None,
        );
        g.assert();
        g.assert_len(NBITS_GRAN, 0, 1, 0);
        g.assert_loc(n1, Mat4::from(1.0));
        g.assert_hier(n1, None, vec![]);

        let n2 = g.insert(Node::Xform(Mat4::from(2.0)), None);
        g.assert();
        g.assert_len(NBITS_GRAN, 0, 1, 1);
        g.assert_loc(n2, Mat4::from(2.0));
        g.assert_hier(n2, None, vec![]);

        let n21 = g.insert(
            Node::Light(
                Light::new_white(LightType::Directional, 1000.0),
                Mat4::from(21.0),
            ),
            Some(n2),
        );
        g.assert();
        g.assert_len(NBITS_GRAN, 0, 2, 1);
        g.assert_loc(n21, Mat4::from(21.0));
        g.assert_hier(n21, Some(n2), vec![]);

        g.assert_hier(n2, None, vec![n21]);

        let n211 = g.insert(Node::Xform(Mat4::from(211.0)), Some(n21));
        g.assert();
        g.assert_len(NBITS_GRAN, 0, 2, 2);
        g.assert_loc(n211, Mat4::from(211.0));
        g.assert_hier(n211, Some(n21), vec![]);

        g.assert_hier(n2, None, vec![n21]);
        g.assert_hier(n21, Some(n2), vec![n211]);

        let n3 = g.insert(Node::Xform(Mat4::from(3.0)), None);
        g.assert();
        g.assert_len(NBITS_GRAN, 0, 2, 3);
        g.assert_loc(n3, Mat4::from(3.0));
        g.assert_hier(n3, None, vec![]);

        let n11 = g.insert(Node::Xform(Mat4::from(11.0)), Some(n1));
        g.assert();
        g.assert_len(NBITS_GRAN, 0, 2, 4);
        g.assert_loc(n11, Mat4::from(11.0));
        g.assert_hier(n11, Some(n1), vec![]);

        g.assert_hier(n1, None, vec![n11]);

        let n212 = g.insert(
            Node::Light(
                Light::new_white(LightType::Directional, 650.0),
                Mat4::from(212.0),
            ),
            Some(n21),
        );
        g.assert();
        g.assert_len(NBITS_GRAN, 0, 3, 4);
        g.assert_loc(n212, Mat4::from(212.0));
        g.assert_hier(n212, Some(n21), vec![]);

        g.assert_hier(n21, Some(n2), vec![n212, n211]);
    }

    #[test]
    fn remove_one() {
        // TODO:  Node::Drawable.

        let mut g = Graph::new();
        let n = g.insert(
            Node::Light(
                Light::new_white(LightType::Directional, 500.0),
                Mat4::from(0.5),
            ),
            None,
        );
        let n = g.remove(n);
        g.assert();
        g.assert_len(NBITS_GRAN, 0, 0, 0);
        match n {
            Node::Light(l, x) => {
                assert_eq!(l.intensity(), 500.0);
                assert_eq!(x, Mat4::from(0.5));
            }
            _ => assert!(false),
        }

        let mut g = Graph::new();
        let n = g.insert(Node::Xform(Mat4::from(0.25)), None);
        let n = g.remove(n);
        g.assert();
        g.assert_len(NBITS_GRAN, 0, 0, 0);
        match n {
            Node::Xform(x) => assert_eq!(x, Mat4::from(0.25)),
            _ => assert!(false),
        }
    }

    #[test]
    fn remove() {
        let mut g = Graph::new();

        let n1 = g.insert(
            Node::Light(
                Light::new_white(LightType::Directional, 500.0),
                Mat4::from(1.0),
            ),
            None,
        );

        let n2 = g.insert(Node::Xform(Mat4::from(2.0)), None);

        let n21 = g.insert(
            Node::Light(
                Light::new_white(LightType::Directional, 1000.0),
                Mat4::from(21.0),
            ),
            Some(n2),
        );

        let n211 = g.insert(Node::Xform(Mat4::from(211.0)), Some(n21));

        let n3 = g.insert(Node::Xform(Mat4::from(3.0)), None);

        let n11 = g.insert(Node::Xform(Mat4::from(11.0)), Some(n1));

        let n212 = g.insert(
            Node::Light(
                Light::new_white(LightType::Directional, 650.0),
                Mat4::from(212.0),
            ),
            Some(n21),
        );

        g.assert();
        g.assert_len(NBITS_GRAN, 0, 3, 4);
        g.assert_hier(n1, None, vec![n11]);
        g.assert_hier(n11, Some(n1), vec![]);
        g.assert_hier(n2, None, vec![n21]);
        g.assert_hier(n21, Some(n2), vec![n211, n212]);
        g.assert_hier(n211, Some(n21), vec![]);
        g.assert_hier(n212, Some(n21), vec![]);
        g.assert_hier(n3, None, vec![]);

        let n3 = g.remove(n3);
        g.assert();
        g.assert_len(NBITS_GRAN, 0, 3, 3);
        match n3 {
            Node::Xform(x) => assert_eq!(x, Mat4::from(3.0)),
            _ => assert!(false),
        }

        let n1 = g.remove(n1);
        g.assert();
        g.assert_len(NBITS_GRAN, 0, 2, 3);
        match n1 {
            Node::Light(l, x) => {
                assert_eq!(l.intensity(), 500.0);
                assert_eq!(x, Mat4::from(1.0));
            }
            _ => assert!(false),
        }
        g.assert_hier(n11, None, vec![]);

        let n21 = g.remove(n21);
        g.assert();
        g.assert_len(NBITS_GRAN, 0, 1, 3);
        match n21 {
            Node::Light(l, x) => {
                assert_eq!(l.intensity(), 1000.0);
                assert_eq!(x, Mat4::from(21.0));
            }
            _ => assert!(false),
        }
        g.assert_hier(n2, None, vec![n211, n212]);
        g.assert_hier(n211, Some(n2), vec![]);
        g.assert_hier(n212, Some(n2), vec![]);

        let n211 = g.remove(n211);
        g.assert();
        g.assert_len(NBITS_GRAN, 0, 1, 2);
        match n211 {
            Node::Xform(x) => assert_eq!(x, Mat4::from(211.0)),
            _ => assert!(false),
        }
        g.assert_hier(n2, None, vec![n212]);
        g.assert_hier(n212, Some(n2), vec![]);

        let n2 = g.remove(n2);
        g.assert();
        g.assert_len(NBITS_GRAN, 0, 1, 1);
        match n2 {
            Node::Xform(x) => assert_eq!(x, Mat4::from(2.0)),
            _ => assert!(false),
        }
        g.assert_hier(n212, None, vec![]);

        let n11 = g.remove(n11);
        g.assert();
        g.assert_len(NBITS_GRAN, 0, 1, 0);
        match n11 {
            Node::Xform(x) => assert_eq!(x, Mat4::from(11.0)),
            _ => assert!(false),
        }

        let n212 = g.remove(n212);
        g.assert();
        g.assert_len(NBITS_GRAN, 0, 0, 0);
        match n212 {
            Node::Light(l, x) => {
                assert_eq!(l.intensity(), 650.0);
                assert_eq!(x, Mat4::from(212.0));
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn insert_remove() {
        let mut g = Graph::new();

        let n1 = g.insert(
            Node::Light(
                Light::new_white(LightType::Directional, 500.0),
                Mat4::from(1.0),
            ),
            None,
        );

        let n2 = g.insert(Node::Xform(Mat4::from(2.0)), None);

        let n21 = g.insert(
            Node::Light(
                Light::new_white(LightType::Directional, 1000.0),
                Mat4::from(21.0),
            ),
            Some(n2),
        );

        let n211 = g.insert(Node::Xform(Mat4::from(211.0)), Some(n21));

        let n3 = g.insert(Node::Xform(Mat4::from(3.0)), None);

        let n11 = g.insert(Node::Xform(Mat4::from(11.0)), Some(n1));

        let n212 = g.insert(
            Node::Light(
                Light::new_white(LightType::Directional, 650.0),
                Mat4::from(212.0),
            ),
            Some(n21),
        );

        g.assert();
        g.assert_len(NBITS_GRAN, 0, 3, 4);
        g.assert_hier(n1, None, vec![n11]);
        g.assert_hier(n11, Some(n1), vec![]);
        g.assert_hier(n2, None, vec![n21]);
        g.assert_hier(n21, Some(n2), vec![n211, n212]);
        g.assert_hier(n211, Some(n21), vec![]);
        g.assert_hier(n212, Some(n21), vec![]);
        g.assert_hier(n3, None, vec![]);

        let n3 = g.remove(n3);
        let n3 = g.insert(Node::Xform(Mat4::from(3.0)), None);
        g.assert();
        g.assert_len(NBITS_GRAN, 0, 3, 4);
        g.assert_loc(n3, Mat4::from(3.0));
        g.assert_hier(n3, None, vec![]);
        let n3 = g.remove(n3);
        g.assert();
        g.assert_len(NBITS_GRAN, 0, 3, 3);
        match n3 {
            Node::Xform(x) => assert_eq!(x, Mat4::from(3.0)),
            _ => assert!(false),
        }

        let n1 = g.remove(n1);
        let n111 = g.insert(
            Node::Light(
                Light::new_white(LightType::Directional, 900.0),
                Mat4::from(111.0),
            ),
            Some(n11),
        );
        let n112 = g.insert(
            Node::Light(
                Light::new_white(LightType::Directional, 380.0),
                Mat4::from(112.0),
            ),
            Some(n11),
        );
        g.assert();
        g.assert_len(NBITS_GRAN, 0, 4, 3);
        g.assert_loc(n111, Mat4::from(111.0));
        g.assert_loc(n112, Mat4::from(112.0));
        g.assert_hier(n11, None, vec![n111, n112]);
        g.assert_hier(n111, Some(n11), vec![]);
        g.assert_hier(n112, Some(n11), vec![]);

        let n213 = g.insert(
            Node::Light(
                Light::new_white(LightType::Directional, 440.0),
                Mat4::from(213.0),
            ),
            Some(n21),
        );
        let n2131 = g.insert(
            Node::Light(
                Light::new_white(LightType::Directional, 100.0),
                Mat4::from(2131.0),
            ),
            Some(n213),
        );
        let n21 = g.remove(n21);
        g.assert();
        g.assert_len(NBITS_GRAN, 0, 5, 3);
        g.assert_loc(n213, Mat4::from(213.0));
        g.assert_loc(n2131, Mat4::from(2131.0));
        g.assert_hier(n2, None, vec![n211, n212, n213]);
        g.assert_hier(n211, Some(n2), vec![]);
        g.assert_hier(n212, Some(n2), vec![]);
        g.assert_hier(n213, Some(n2), vec![n2131]);
        g.assert_hier(n2131, Some(n213), vec![]);

        let n21 = g.insert(Node::Xform(Mat4::from(21.0)), Some(n2));
        g.assert();
        g.assert_len(NBITS_GRAN, 0, 5, 4);
        g.assert_loc(n21, Mat4::from(21.0));
        g.assert_hier(n2, None, vec![n211, n212, n213, n21]);
        g.assert_hier(n21, Some(n2), vec![]);

        let n2132 = g.insert(
            Node::Light(
                Light::new_white(LightType::Directional, 1260.0),
                Mat4::from(2132.0),
            ),
            Some(n213),
        );
        let n21321 = g.insert(Node::Xform(Mat4::from(21321.0)), Some(n2132));
        let n21322 = g.insert(Node::Xform(Mat4::from(21322.0)), Some(n2132));
        g.assert();
        g.assert_len(NBITS_GRAN, 0, 6, 6);
        g.assert_loc(n2132, Mat4::from(2132.0));
        g.assert_loc(n21321, Mat4::from(21321.0));
        g.assert_loc(n21322, Mat4::from(21322.0));
        g.assert_hier(n213, Some(n2), vec![n2131, n2132]);
        g.assert_hier(n2132, Some(n213), vec![n21321, n21322]);
        g.assert_hier(n21321, Some(n2132), vec![]);
        g.assert_hier(n21322, Some(n2132), vec![]);

        let n213 = g.remove(n213);
        g.assert();
        g.assert_len(NBITS_GRAN, 0, 5, 6);
        g.assert_hier(n2, None, vec![n211, n212, n21, n2131, n2132]);
        g.assert_hier(n2131, Some(n2), vec![]);
        g.assert_hier(n2132, Some(n2), vec![n21321, n21322]);
        g.assert_hier(n21321, Some(n2132), vec![]);
        g.assert_hier(n21322, Some(n2132), vec![]);
        match n213 {
            Node::Light(l, x) => {
                assert_eq!(l.intensity(), 440.0);
                assert_eq!(x, Mat4::from(213.0));
            }
            _ => assert!(false),
        }

        let n2 = g.remove(n2);
        g.assert();
        g.assert_len(NBITS_GRAN, 0, 5, 5);
        g.assert_hier(n21, None, vec![]);
        g.assert_hier(n211, None, vec![]);
        g.assert_hier(n212, None, vec![]);
        g.assert_hier(n2131, None, vec![]);
        g.assert_hier(n2132, None, vec![n21321, n21322]);
        g.assert_hier(n21321, Some(n2132), vec![]);
        g.assert_hier(n21322, Some(n2132), vec![]);
        match n2 {
            Node::Xform(x) => assert_eq!(x, Mat4::from(2.0)),
            _ => assert!(false),
        }

        let n11 = g.remove(n11);
        let n211 = g.remove(n211);
        let n212 = g.remove(n212);
        g.assert_len(NBITS_GRAN, 0, 4, 3);
        g.assert_hier(n111, None, vec![]);
        g.assert_hier(n112, None, vec![]);
        match n11 {
            Node::Xform(x) => assert_eq!(x, Mat4::from(11.0)),
            _ => assert!(false),
        }
        match n211 {
            Node::Xform(x) => assert_eq!(x, Mat4::from(211.0)),
            _ => assert!(false),
        }
        match n212 {
            Node::Light(l, x) => {
                assert_eq!(l.intensity(), 650.0);
                assert_eq!(x, Mat4::from(212.0));
            }
            _ => assert!(false),
        }

        let n21321 = g.remove(n21321);
        g.assert();
        g.assert_len(NBITS_GRAN, 0, 4, 2);
        g.assert_hier(n2131, None, vec![]);
        g.assert_hier(n2132, None, vec![n21322]);
        g.assert_hier(n21322, Some(n2132), vec![]);
        match n21321 {
            Node::Xform(x) => assert_eq!(x, Mat4::from(21321.0)),
            _ => assert!(false),
        }

        let n2132 = g.remove(n2132);
        g.assert();
        g.assert_len(NBITS_GRAN, 0, 3, 2);
        g.assert_hier(n2131, None, vec![]);
        g.assert_hier(n21322, None, vec![]);
        match n2132 {
            Node::Light(l, x) => {
                assert_eq!(l.intensity(), 1260.0);
                assert_eq!(x, Mat4::from(2132.0));
            }
            _ => assert!(false),
        }

        let n112 = g.remove(n112);
        let n111 = g.remove(n111);
        g.assert_len(NBITS_GRAN, 0, 1, 2);
        match n112 {
            Node::Light(l, x) => {
                assert_eq!(l.intensity(), 380.0);
                assert_eq!(x, Mat4::from(112.0));
            }
            _ => assert!(false),
        }
        match n111 {
            Node::Light(l, x) => {
                assert_eq!(l.intensity(), 900.0);
                assert_eq!(x, Mat4::from(111.0));
            }
            _ => assert!(false),
        }

        let n2131 = g.remove(n2131);
        let n21322 = g.remove(n21322);
        g.assert();
        g.assert_len(NBITS_GRAN, 0, 0, 1);
        match n2131 {
            Node::Light(l, x) => {
                assert_eq!(l.intensity(), 100.0);
                assert_eq!(x, Mat4::from(2131.0));
            }
            _ => assert!(false),
        }
        match n21322 {
            Node::Xform(x) => assert_eq!(x, Mat4::from(21322.0)),
            _ => assert!(false),
        }

        let n211 = g.insert(
            Node::Light(
                Light::new_white(LightType::Directional, 225.0),
                Mat4::from(211.0),
            ),
            Some(n21),
        );
        g.assert();
        g.assert_len(NBITS_GRAN, 0, 1, 1);
        g.assert_loc(n211, Mat4::from(211.0));
        g.assert_hier(n21, None, vec![n211]);
        g.assert_hier(n211, Some(n21), vec![]);

        let n211 = g.remove(n211);
        g.assert();
        g.assert_len(NBITS_GRAN, 0, 0, 1);
        g.assert_hier(n21, None, vec![]);

        let n21 = g.remove(n21);
        g.assert();
        g.assert_len(NBITS_GRAN, 0, 0, 0);
        match n21 {
            Node::Xform(x) => assert_eq!(x, Mat4::from(21.0)),
            _ => assert!(false),
        }
    }

    #[test]
    fn insert_remove_depth_top() {
        const N: usize = 1000;
        let mut g = Graph::new();

        let light = |x| {
            Node::Light(
                Light::new(
                    LightType::Point { range: x },
                    x,
                    Vec3::new(0.75, 0.325, 0.125),
                ),
                Mat4::from(x),
            )
        };
        let xform = |x: f32| Node::Xform(Mat4::from(x));

        let node = g.insert(light(0.0), None);
        let mut len = (NBITS_GRAN, 0, 1, 0);

        let mut last = node;
        for i in 1..=N {
            let nd = if i & 2 == 0 {
                len.2 += 1;
                g.insert(light(i as _), Some(last))
            } else {
                len.3 += 1;
                g.insert(xform(i as _), Some(last))
            };
            len.0 = g.nbits.len();
            g.assert();
            g.assert_len(len.0, len.1, len.2, len.3);
            g.assert_loc(nd, Mat4::from(i as f32));
            g.assert_hier(nd, Some(last), vec![]);
            last = nd;
        }

        let mut infr = g.children(node)[0];
        let mut node = g.remove(node);
        for i in 0.. {
            len.0 = g.nbits.len();
            let x = match node {
                Node::Drawable(_, x) => {
                    len.1 -= 1;
                    x
                }
                Node::Light(_, x) => {
                    len.2 -= 1;
                    x
                }
                Node::Xform(x) => {
                    len.3 -= 1;
                    x
                }
            };
            assert_eq!(x, Mat4::from((i) as f32));
            g.assert();
            g.assert_len(len.0, len.1, len.2, len.3);
            if let Some(x) = g.children(infr).first() {
                node = g.remove(infr);
                infr = *x;
            } else {
                break;
            }
        }

        assert_eq!(last.0, infr.0);
        g.assert_hier(infr, None, vec![]);
        match g.remove(infr) {
            Node::Light(l, x) => {
                let (lc, xc) = if let Node::Light(lc, xc) = light(1000.0) {
                    (lc, xc)
                } else {
                    panic!();
                };
                assert_eq!(l.intensity(), lc.intensity());
                assert_eq!(x, xc);
            }
            _ => assert!(false),
        };
        g.assert();
        g.assert_len(len.0, 0, 0, 0);
    }

    #[test]
    fn insert_remove_depth_bot() {
        const N: usize = 1000;
        let mut g = Graph::new();

        let light = |x| {
            Node::Light(
                Light::new(
                    LightType::Point { range: x },
                    x,
                    Vec3::new(0.75, 0.325, 0.125),
                ),
                Mat4::from(x),
            )
        };
        let xform = |x: f32| Node::Xform(Mat4::from(x));

        let node = g.insert(light(0.0), None);
        let mut len = (NBITS_GRAN, 0, 1, 0);

        let last = {
            let mut node = node;
            for i in 1..=N {
                let nd = if i & 3 != 0 {
                    len.2 += 1;
                    g.insert(light(i as _), Some(node))
                } else {
                    len.3 += 1;
                    g.insert(xform(i as _), Some(node))
                };
                len.0 = g.nbits.len();
                g.assert();
                g.assert_len(len.0, len.1, len.2, len.3);
                g.assert_loc(nd, Mat4::from(i as f32));
                g.assert_hier(nd, Some(node), vec![]);
                node = nd;
            }
            node
        };

        let first = {
            let mut supr = g.parent(last);
            let mut node = g.remove(last);
            for i in 0.. {
                len.0 = g.nbits.len();
                let x = match node {
                    Node::Drawable(_, x) => {
                        len.1 -= 1;
                        x
                    }
                    Node::Light(_, x) => {
                        len.2 -= 1;
                        x
                    }
                    Node::Xform(x) => {
                        len.3 -= 1;
                        x
                    }
                };
                assert_eq!(x, Mat4::from((N - i) as f32));
                g.assert();
                g.assert_len(len.0, len.1, len.2, len.3);
                if let Some(x) = g.parent(supr.unwrap()) {
                    node = g.remove(supr.unwrap());
                    supr = Some(x);
                } else {
                    break;
                }
            }
            supr.unwrap()
        };

        assert_eq!(first.0, node.0);
        g.assert_hier(first, None, vec![]);
        match g.remove(first) {
            Node::Light(l, x) => {
                let (lc, xc) = if let Node::Light(lc, xc) = light(0.0) {
                    (lc, xc)
                } else {
                    panic!();
                };
                assert_eq!(l.intensity(), lc.intensity());
                assert_eq!(x, xc);
            }
            _ => assert!(false),
        };
        g.assert();
        g.assert_len(len.0, 0, 0, 0);
    }

    #[test]
    fn insert_remove_breadth_fwd() {
        const N: usize = 1000;
        let mut g = Graph::new();

        let light = |x| {
            Node::Light(
                Light::new(LightType::Point { range: x }, x, Vec3::new(0.9, 0.8, 0.7)),
                Mat4::from(x),
            )
        };
        let xform = |x: f32| Node::Xform(Mat4::from(x));

        let node = g.insert(xform(0.0), None);
        let mut chdn = Vec::with_capacity(N);
        let mut len = (NBITS_GRAN, 0, 0, 1);

        for i in 1..=N {
            chdn.push(if i & 6 == 0 {
                len.2 += 1;
                g.insert(light(i as _), Some(node))
            } else {
                len.3 += 1;
                g.insert(xform(i as _), Some(node))
            });
            len.0 = g.nbits.len();
            g.assert();
            g.assert_len(len.0, len.1, len.2, len.3);
            g.assert_loc(*chdn.last().unwrap(), Mat4::from(i as f32));
            g.assert_hier(*chdn.last().unwrap(), Some(node), vec![]);
            g.assert_hier(node, None, chdn.clone());
        }

        for (i, nd) in chdn.iter().enumerate() {
            len.0 = g.nbits.len();
            let x = match g.remove(*nd) {
                Node::Drawable(_, x) => {
                    len.1 -= 1;
                    x
                }
                Node::Light(_, x) => {
                    len.2 -= 1;
                    x
                }
                Node::Xform(x) => {
                    len.3 -= 1;
                    x
                }
            };
            assert_eq!(x, Mat4::from((i + 1) as f32));
            g.assert();
            g.assert_len(len.0, len.1, len.2, len.3);
            g.assert_hier(node, None, chdn[i + 1..].to_vec());
        }

        g.assert_hier(node, None, vec![]);
        match g.remove(node) {
            Node::Xform(x) => {
                let xc = if let Node::Xform(xc) = xform(0.0) {
                    xc
                } else {
                    panic!();
                };
                assert_eq!(x, xc);
            }
            _ => assert!(false),
        };
        g.assert();
        g.assert_len(len.0, 0, 0, 0);
    }

    #[test]
    fn insert_remove_breadth_bwd() {
        const N: usize = 1000;
        let mut g = Graph::new();

        let light = |x| {
            Node::Light(
                Light::new(LightType::Point { range: x }, x, Vec3::new(0.9, 0.8, 0.7)),
                Mat4::from(x),
            )
        };
        let xform = |x: f32| Node::Xform(Mat4::from(x));

        let node = g.insert(xform(0.0), None);
        let mut chdn = Vec::with_capacity(N);
        let mut len = (NBITS_GRAN, 0, 0, 1);

        for i in 1..=N {
            chdn.push(if i & 5 != 0 {
                len.2 += 1;
                g.insert(light(i as _), Some(node))
            } else {
                len.3 += 1;
                g.insert(xform(i as _), Some(node))
            });
            len.0 = g.nbits.len();
            g.assert();
            g.assert_len(len.0, len.1, len.2, len.3);
            g.assert_loc(*chdn.last().unwrap(), Mat4::from(i as f32));
            g.assert_hier(*chdn.last().unwrap(), Some(node), vec![]);
            g.assert_hier(node, None, chdn.clone());
        }

        let mut i = 0;
        while let Some(nd) = chdn.pop() {
            len.0 = g.nbits.len();
            let x = match g.remove(nd) {
                Node::Drawable(_, x) => {
                    len.1 -= 1;
                    x
                }
                Node::Light(_, x) => {
                    len.2 -= 1;
                    x
                }
                Node::Xform(x) => {
                    len.3 -= 1;
                    x
                }
            };
            assert_eq!(x, Mat4::from((N - i) as f32));
            i += 1;
            g.assert();
            g.assert_len(len.0, len.1, len.2, len.3);
            g.assert_hier(node, None, chdn.clone());
        }

        g.assert_hier(node, None, vec![]);
        match g.remove(node) {
            Node::Xform(x) => {
                let xc = if let Node::Xform(xc) = xform(0.0) {
                    xc
                } else {
                    panic!();
                };
                assert_eq!(x, xc);
            }
            _ => assert!(false),
        };
        g.assert();
        g.assert_len(len.0, 0, 0, 0);
    }
}
