// Copyright 2023 Gustavo C. Viegas. All rights reserved.

use crate::light::LightType;

use super::*;

impl Graph {
    fn assert(&self) {
        assert_eq!(self.nodes.len(), self.nbits.len());
        assert_eq!(self.len(), self.nbits.len() - self.nbits.rem());
        for (i, x) in self.nodes.iter().enumerate() {
            assert_ne!(i, x.next);
            assert_ne!(i, x.prev);
            assert_ne!(i, x.sub);
            assert!(x.next == NONE || x.next < self.nodes.len());
            assert!(x.prev == NONE || x.prev < self.nodes.len());
            assert!(x.sub == NONE || x.sub < self.nodes.len());
            if self.nbits.is_set(i) {
                assert!(x.data != NONE && x.data < self.len());
            }
        }
    }

    fn assert_loc(&self, node: NodeId, local: Mat4<f32>) {
        assert_eq!(&local, self.local(node));
    }

    fn assert_unconn(&self, node: NodeId) {
        let &NodeLink {
            next,
            prev,
            sub,
            data,
        } = &self.nodes[node.0];
        assert_eq!(next, NONE);
        assert_eq!(prev, NONE);
        assert_eq!(sub, NONE);
        assert!(data != NONE && data < self.len());
    }

    fn assert_hier(&self, node: NodeId, parent: Option<NodeId>, mut children: Vec<NodeId>) {
        let pnt = match self.nodes[node.0].prev {
            NONE => NONE,
            mut prev => {
                let mut pprev = prev;
                prev = node.0;
                while self.nodes[pprev].sub != prev {
                    prev = pprev;
                    pprev = self.nodes[pprev].prev;
                }
                assert_eq!(self.nodes[pprev].sub, prev);
                pprev
            }
        };
        assert_eq!(pnt, parent.map_or(NONE, |x| x.0));

        let mut chdn = match self.nodes[node.0].sub {
            NONE => vec![],
            sub => {
                let mut chdn = vec![NodeId(sub)];
                let mut next = self.nodes[sub].next;
                while next != NONE {
                    chdn.push(NodeId(next));
                    next = self.nodes[next].next;
                }
                chdn
            }
        };
        assert_eq!(chdn.len(), children.len());
        chdn.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        children.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        chdn.into_iter()
            .zip(children)
            .for_each(|(a, b)| assert_eq!(a.0, b.0));
    }

    fn assert_wld(&self, node: NodeId) {
        let world = self.world(node);
        let mut x = self.nodes[node.0].sub;
        while x != NONE {
            assert_eq!(*self.world(NodeId(x)), world * self.local(NodeId(x)));
            self.assert_wld(NodeId(x));
            x = self.nodes[x].next;
        }
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
    assert_eq!(g.len(), 1);
    g.assert_loc(n, Mat4::from(1.5));
    g.assert_unconn(n);
    g.assert_hier(n, None, vec![]);

    let mut g = Graph::new();
    let n = g.insert(Node::Xform(Mat4::from(-1.0)), None);
    g.assert();
    assert_eq!(g.len(), 1);
    g.assert_loc(n, Mat4::from(-1.0));
    g.assert_unconn(n);
    g.assert_hier(n, None, vec![]);
}

#[test]
fn insert() {
    let mut g = Graph::new();
    g.assert();
    assert_eq!(g.len(), 0);

    let n1 = g.insert(
        Node::Light(
            Light::new_white(LightType::Directional, 500.0),
            Mat4::from(1.0),
        ),
        None,
    );
    g.assert();
    assert_eq!(g.len(), 1);
    g.assert_loc(n1, Mat4::from(1.0));
    g.assert_hier(n1, None, vec![]);

    let n2 = g.insert(Node::Xform(Mat4::from(2.0)), None);
    g.assert();
    assert_eq!(g.len(), 2);
    g.assert_loc(n1, Mat4::from(1.0));
    g.assert_loc(n2, Mat4::from(2.0));
    g.assert_hier(n1, None, vec![]);
    g.assert_hier(n2, None, vec![]);

    let n21 = g.insert(
        Node::Light(
            Light::new_white(LightType::Directional, 1000.0),
            Mat4::from(21.0),
        ),
        Some(n2),
    );
    g.assert();
    assert_eq!(g.len(), 3);
    g.assert_loc(n1, Mat4::from(1.0));
    g.assert_loc(n2, Mat4::from(2.0));
    g.assert_loc(n21, Mat4::from(21.0));
    g.assert_hier(n1, None, vec![]);
    g.assert_hier(n2, None, vec![n21]);
    g.assert_hier(n21, Some(n2), vec![]);

    g.assert_hier(n2, None, vec![n21]);

    let n211 = g.insert(Node::Xform(Mat4::from(211.0)), Some(n21));
    g.assert();
    assert_eq!(g.len(), 4);
    g.assert_loc(n1, Mat4::from(1.0));
    g.assert_loc(n2, Mat4::from(2.0));
    g.assert_loc(n21, Mat4::from(21.0));
    g.assert_loc(n211, Mat4::from(211.0));
    g.assert_hier(n1, None, vec![]);
    g.assert_hier(n2, None, vec![n21]);
    g.assert_hier(n21, Some(n2), vec![n211]);
    g.assert_hier(n211, Some(n21), vec![]);

    let n3 = g.insert(Node::Xform(Mat4::from(3.0)), None);
    g.assert();
    assert_eq!(g.len(), 5);
    g.assert_loc(n1, Mat4::from(1.0));
    g.assert_loc(n2, Mat4::from(2.0));
    g.assert_loc(n21, Mat4::from(21.0));
    g.assert_loc(n211, Mat4::from(211.0));
    g.assert_loc(n3, Mat4::from(3.0));
    g.assert_hier(n1, None, vec![]);
    g.assert_hier(n2, None, vec![n21]);
    g.assert_hier(n21, Some(n2), vec![n211]);
    g.assert_hier(n211, Some(n21), vec![]);
    g.assert_hier(n3, None, vec![]);

    let n11 = g.insert(Node::Xform(Mat4::from(11.0)), Some(n1));
    g.assert();
    assert_eq!(g.len(), 6);
    g.assert_loc(n1, Mat4::from(1.0));
    g.assert_loc(n11, Mat4::from(11.0));
    g.assert_loc(n2, Mat4::from(2.0));
    g.assert_loc(n21, Mat4::from(21.0));
    g.assert_loc(n211, Mat4::from(211.0));
    g.assert_loc(n3, Mat4::from(3.0));
    g.assert_hier(n1, None, vec![n11]);
    g.assert_hier(n11, Some(n1), vec![]);
    g.assert_hier(n2, None, vec![n21]);
    g.assert_hier(n21, Some(n2), vec![n211]);
    g.assert_hier(n211, Some(n21), vec![]);
    g.assert_hier(n3, None, vec![]);

    let n212 = g.insert(
        Node::Light(
            Light::new_white(LightType::Directional, 650.0),
            Mat4::from(212.0),
        ),
        Some(n21),
    );
    g.assert();
    assert_eq!(g.len(), 7);
    g.assert_loc(n1, Mat4::from(1.0));
    g.assert_loc(n11, Mat4::from(11.0));
    g.assert_loc(n2, Mat4::from(2.0));
    g.assert_loc(n21, Mat4::from(21.0));
    g.assert_loc(n211, Mat4::from(211.0));
    g.assert_loc(n212, Mat4::from(212.0));
    g.assert_loc(n3, Mat4::from(3.0));
    g.assert_hier(n1, None, vec![n11]);
    g.assert_hier(n11, Some(n1), vec![]);
    g.assert_hier(n2, None, vec![n21]);
    g.assert_hier(n21, Some(n2), vec![n212, n211]);
    g.assert_hier(n211, Some(n21), vec![]);
    g.assert_hier(n212, Some(n21), vec![]);
    g.assert_hier(n3, None, vec![]);
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
    let mut n = g.remove(n);
    assert_eq!(n.len(), 1);
    match n.pop().unwrap() {
        Node::Light(l, x) => {
            assert_eq!(l.intensity(), 500.0);
            assert_eq!(x, Mat4::from(0.5));
        }
        _ => panic!(),
    }
    g.assert();
    assert_eq!(g.len(), 0);

    let mut g = Graph::new();
    let n = g.insert(Node::Xform(Mat4::from(0.25)), None);
    let mut n = g.remove(n);
    assert_eq!(n.len(), 1);
    match n.pop().unwrap() {
        Node::Xform(x) => assert_eq!(x, Mat4::from(0.25)),
        _ => panic!(),
    }
    g.assert();
    assert_eq!(g.len(), 0);
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
    assert_eq!(g.len(), 7);
    g.assert_hier(n1, None, vec![n11]);
    g.assert_hier(n11, Some(n1), vec![]);
    g.assert_hier(n2, None, vec![n21]);
    g.assert_hier(n21, Some(n2), vec![n211, n212]);
    g.assert_hier(n211, Some(n21), vec![]);
    g.assert_hier(n212, Some(n21), vec![]);
    g.assert_hier(n3, None, vec![]);

    let mut n3 = g.remove(n3);
    assert_eq!(n3.len(), 1);
    let n3 = n3.pop().unwrap();
    match n3 {
        Node::Xform(x) => assert_eq!(x, Mat4::from(3.0)),
        _ => panic!(),
    }
    g.assert();
    assert_eq!(g.len(), 6);
    g.assert_hier(n1, None, vec![n11]);
    g.assert_hier(n11, Some(n1), vec![]);
    g.assert_hier(n2, None, vec![n21]);
    g.assert_hier(n21, Some(n2), vec![n211, n212]);
    g.assert_hier(n211, Some(n21), vec![]);
    g.assert_hier(n212, Some(n21), vec![]);

    let mut n1 = g.remove(n1);
    assert_eq!(n1.len(), 2);
    let n11 = n1.pop().unwrap();
    let n1 = n1.pop().unwrap();
    match n1 {
        Node::Light(l, x) => {
            assert_eq!(l.intensity(), 500.0);
            assert_eq!(x, Mat4::from(1.0));
        }
        _ => panic!(),
    }
    match n11 {
        Node::Xform(x) => assert_eq!(x, Mat4::from(11.0)),
        _ => panic!(),
    }
    g.assert();
    assert_eq!(g.len(), 4);
    g.assert_hier(n2, None, vec![n21]);
    g.assert_hier(n21, Some(n2), vec![n211, n212]);
    g.assert_hier(n211, Some(n21), vec![]);
    g.assert_hier(n212, Some(n21), vec![]);

    let mut n21 = g.remove(n21);
    assert_eq!(n21.len(), 3);
    n21.drain(1..3).for_each(|n| match n {
        Node::Light(l, x) => {
            assert_eq!(l.intensity(), 650.0);
            assert_eq!(x, Mat4::from(212.0));
        }
        Node::Xform(x) => assert_eq!(x, Mat4::from(211.0)),
        _ => panic!(),
    });
    match n21.pop().unwrap() {
        Node::Light(l, x) => {
            assert_eq!(l.intensity(), 1000.0);
            assert_eq!(x, Mat4::from(21.0));
        }
        _ => panic!(),
    }
    g.assert();
    assert_eq!(g.len(), 1);
    g.assert_hier(n2, None, vec![]);

    let mut n2 = g.remove(n2);
    assert_eq!(n2.len(), 1);
    let n2 = n2.pop().unwrap();
    match n2 {
        Node::Xform(x) => assert_eq!(x, Mat4::from(2.0)),
        _ => panic!(),
    }
    g.assert();
    assert_eq!(g.len(), 0);
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
    assert_eq!(g.len(), 7);
    g.assert_hier(n1, None, vec![n11]);
    g.assert_hier(n11, Some(n1), vec![]);
    g.assert_hier(n2, None, vec![n21]);
    g.assert_hier(n21, Some(n2), vec![n211, n212]);
    g.assert_hier(n211, Some(n21), vec![]);
    g.assert_hier(n212, Some(n21), vec![]);
    g.assert_hier(n3, None, vec![]);

    let n3 = g.remove(n3).pop().unwrap();
    let n3 = g.insert(n3, None);
    g.assert();
    assert_eq!(g.len(), 7);
    g.assert_hier(n1, None, vec![n11]);
    g.assert_hier(n11, Some(n1), vec![]);
    g.assert_hier(n2, None, vec![n21]);
    g.assert_hier(n21, Some(n2), vec![n211, n212]);
    g.assert_hier(n211, Some(n21), vec![]);
    g.assert_hier(n212, Some(n21), vec![]);
    g.assert_hier(n3, None, vec![]);
    let n3 = g.remove(n3).pop().unwrap();
    match n3 {
        Node::Xform(x) => assert_eq!(x, Mat4::from(3.0)),
        _ => panic!(),
    }
    g.assert();
    assert_eq!(g.len(), 6);
    g.assert_hier(n1, None, vec![n11]);
    g.assert_hier(n11, Some(n1), vec![]);
    g.assert_hier(n2, None, vec![n21]);
    g.assert_hier(n21, Some(n2), vec![n211, n212]);
    g.assert_hier(n211, Some(n21), vec![]);
    g.assert_hier(n212, Some(n21), vec![]);

    let mut n1 = g.remove(n1);
    let n11 = n1.pop().unwrap();
    let n1 = g.insert(n1.pop().unwrap(), None);
    let n11 = g.insert(n11, Some(n1));
    g.assert();
    assert_eq!(g.len(), 6);
    g.assert_hier(n1, None, vec![n11]);
    g.assert_hier(n11, Some(n1), vec![]);
    g.assert_hier(n2, None, vec![n21]);
    g.assert_hier(n21, Some(n2), vec![n211, n212]);
    g.assert_hier(n211, Some(n21), vec![]);
    g.assert_hier(n212, Some(n21), vec![]);
    let mut n1 = g.remove(n1);
    let n11 = n1.pop().unwrap();
    let n1 = n1.pop().unwrap();
    match n1 {
        Node::Light(l, x) => {
            assert_eq!(l.intensity(), 500.0);
            assert_eq!(x, Mat4::from(1.0));
        }
        _ => panic!(),
    }
    match n11 {
        Node::Xform(x) => assert_eq!(x, Mat4::from(11.0)),
        _ => panic!(),
    }
    g.assert();
    assert_eq!(g.len(), 4);
    g.assert_hier(n2, None, vec![n21]);
    g.assert_hier(n21, Some(n2), vec![n211, n212]);
    g.assert_hier(n211, Some(n21), vec![]);
    g.assert_hier(n212, Some(n21), vec![]);

    let n213 = g.insert(
        Node::Light(
            Light::new_white(LightType::Directional, 440.0),
            Mat4::from(213.0),
        ),
        Some(n21),
    );
    let _n2131 = g.insert(
        Node::Light(
            Light::new_white(LightType::Directional, 100.0),
            Mat4::from(2131.0),
        ),
        Some(n213),
    );
    let mut n21 = g.remove(n21);
    assert_eq!(n21.len(), 5);
    n21.drain(1..5).for_each(|n| match n {
        Node::Light(l, x) => match x[0][0] as i32 {
            212 => assert_eq!(l.intensity(), 650.0),
            213 => assert_eq!(l.intensity(), 440.0),
            2131 => assert_eq!(l.intensity(), 100.0),
            _ => panic!(),
        },
        Node::Xform(x) => assert_eq!(x, Mat4::from(211.0)),
        _ => panic!(),
    });
    match n21.pop().unwrap() {
        Node::Light(l, x) => {
            assert_eq!(l.intensity(), 1000.0);
            assert_eq!(x, Mat4::from(21.0));
        }
        _ => panic!(),
    }
    g.assert();
    assert_eq!(g.len(), 1);
    g.assert_hier(n2, None, vec![]);

    let n21 = g.insert(Node::Xform(Mat4::from(21.0)), Some(n2));
    let n211 = g.insert(
        Node::Light(
            Light::new_white(LightType::Directional, 300.0),
            Mat4::from(211.0),
        ),
        Some(n21),
    );
    let n1 = g.insert(
        Node::Light(
            Light::new_white(LightType::Directional, 275.0),
            Mat4::from(1.0),
        ),
        None,
    );
    g.assert();
    assert_eq!(g.len(), 4);
    g.assert_hier(n1, None, vec![]);
    g.assert_hier(n2, None, vec![n21]);
    g.assert_hier(n21, Some(n2), vec![n211]);
    g.assert_hier(n211, Some(n21), vec![]);
    let mut n2 = g.remove(n2);
    assert_eq!(n2.len(), 3);
    n2.drain(1..3).for_each(|n| match n {
        Node::Light(l, x) => {
            assert_eq!(l.intensity(), 300.0);
            assert_eq!(x, Mat4::from(211.0));
        }
        Node::Xform(x) => assert_eq!(x, Mat4::from(21.0)),
        _ => panic!(),
    });
    match n2.pop().unwrap() {
        Node::Xform(x) => assert_eq!(x, Mat4::from(2.0)),
        _ => panic!(),
    }
    g.assert();
    assert_eq!(g.len(), 1);
    g.assert_hier(n1, None, vec![]);
    let mut n1 = g.remove(n1);
    assert_eq!(n1.len(), 1);
    g.assert();
    assert_eq!(g.len(), 0);
    match n1.pop().unwrap() {
        Node::Light(l, x) => {
            assert_eq!(l.intensity(), 275.0);
            assert_eq!(x, Mat4::from(1.0));
        }
        _ => panic!(),
    }

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
    assert_eq!(g.len(), 7);
    g.assert_hier(n1, None, vec![n11]);
    g.assert_hier(n11, Some(n1), vec![]);
    g.assert_hier(n2, None, vec![n21]);
    g.assert_hier(n21, Some(n2), vec![n211, n212]);
    g.assert_hier(n211, Some(n21), vec![]);
    g.assert_hier(n212, Some(n21), vec![]);
    g.assert_hier(n3, None, vec![]);

    const CMP: [f32; 7] = [1.0, 2.0, 3.0, 11.0, 21.0, 211.0, 212.0];

    let mut ns = g
        .remove(n1)
        .into_iter()
        .chain(g.remove(n2))
        .chain(g.remove(n3))
        .collect::<Vec<_>>();
    assert_eq!(ns.len(), 7);
    g.assert();
    assert_eq!(g.len(), 0);
    ns.sort_unstable_by(|a, b| {
        let x = match a {
            Node::Drawable(_, x) => x[0][0],
            Node::Light(_, x) => x[0][0],
            Node::Xform(x) => x[0][0],
        };
        let y = match b {
            Node::Drawable(_, x) => x[0][0],
            Node::Light(_, x) => x[0][0],
            Node::Xform(x) => x[0][0],
        };
        y.partial_cmp(&x).unwrap()
    });
    let mut ndep = vec![g.insert(ns.pop().unwrap(), None)];
    while let Some(n) = ns.pop() {
        ndep.push(g.insert(n, Some(*ndep.last().unwrap())));
    }
    g.assert();
    assert_eq!(g.len(), 7);
    g.assert_hier(ndep[0], None, vec![ndep[1]]);
    g.assert_hier(ndep[1], Some(ndep[0]), vec![ndep[2]]);
    g.assert_hier(ndep[2], Some(ndep[1]), vec![ndep[3]]);
    g.assert_hier(ndep[3], Some(ndep[2]), vec![ndep[4]]);
    g.assert_hier(ndep[4], Some(ndep[3]), vec![ndep[5]]);
    g.assert_hier(ndep[5], Some(ndep[4]), vec![ndep[6]]);
    g.assert_hier(ndep[6], Some(ndep[5]), vec![]);
    CMP.iter().zip(ndep.iter()).for_each(|(&x, n)| {
        assert_eq!(
            x,
            match g.data[g.nodes[n.0].data].data {
                Node::Drawable(_, x) => x[0][0],
                Node::Light(_, x) => x[0][0],
                Node::Xform(x) => x[0][0],
            }
        )
    });

    let ns = g.remove(n1);
    assert_eq!(ns.len(), 7);
    g.assert();
    assert_eq!(g.len(), 0);
    CMP.iter().zip(ns.iter()).for_each(|(&x, n)| {
        assert_eq!(
            x,
            match n {
                Node::Drawable(_, x) => x[0][0],
                Node::Light(_, x) => x[0][0],
                Node::Xform(x) => x[0][0],
            }
        )
    });
    let mut nbdt = vec![];
    for i in ns {
        nbdt.push(g.insert(i, None));
    }
    g.assert();
    assert_eq!(g.len(), 7);
    g.assert_hier(nbdt[0], None, vec![]);
    g.assert_hier(nbdt[1], None, vec![]);
    g.assert_hier(nbdt[2], None, vec![]);
    g.assert_hier(nbdt[3], None, vec![]);
    g.assert_hier(nbdt[4], None, vec![]);
    g.assert_hier(nbdt[5], None, vec![]);
    g.assert_hier(nbdt[6], None, vec![]);
    CMP.iter().zip(nbdt.iter()).for_each(|(&x, n)| {
        assert_eq!(
            x,
            match g.data[g.nodes[n.0].data].data {
                Node::Drawable(_, x) => x[0][0],
                Node::Light(_, x) => x[0][0],
                Node::Xform(x) => x[0][0],
            }
        )
    });

    let mut ns = vec![];
    for i in nbdt {
        ns.append(&mut g.remove(i));
    }
    assert_eq!(ns.len(), 7);
    g.assert();
    assert_eq!(g.len(), 0);
    CMP.iter().zip(ns.iter()).for_each(|(&x, n)| {
        assert_eq!(
            x,
            match n {
                Node::Drawable(_, x) => x[0][0],
                Node::Light(_, x) => x[0][0],
                Node::Xform(x) => x[0][0],
            }
        )
    });
}

#[test]
fn node_growth() {
    let mut g = Graph::new();
    assert_eq!(g.nodes.len(), 0);
    assert_eq!(g.nbits.len(), 0);
    assert_eq!(g.nbits.rem(), 0);

    let node = |x| match x & 1 {
        0 => Node::Xform(Mat4::from(1.0)),
        _ => Node::Light(
            Light::new_white(LightType::Directional, 500.0),
            Mat4::from(1.0),
        ),
    };

    g.insert(node(1), None);
    assert_eq!(g.nodes.len(), NBITS_GRAN);
    assert_eq!(g.nbits.len(), NBITS_GRAN);
    assert_eq!(g.nbits.rem(), NBITS_GRAN - 1);
    let n = g.insert(node(2), None);
    assert_eq!(g.nodes.len(), NBITS_GRAN);
    assert_eq!(g.nbits.len(), NBITS_GRAN);
    assert_eq!(g.nbits.rem(), NBITS_GRAN - 2);
    g.insert(node(3), Some(n));
    assert_eq!(g.nodes.len(), NBITS_GRAN);
    assert_eq!(g.nbits.len(), NBITS_GRAN);
    assert_eq!(g.nbits.rem(), NBITS_GRAN - 3);

    for i in 0..g.nbits.rem() {
        g.insert(node(i), None);
    }
    assert_eq!(g.nodes.len(), NBITS_GRAN);
    assert_eq!(g.nbits.len(), NBITS_GRAN);
    assert_eq!(g.nbits.rem(), 0);
    let mut ns = g.remove(n);
    assert_eq!(g.nodes.len(), NBITS_GRAN);
    assert_eq!(g.nbits.len(), NBITS_GRAN);
    assert_eq!(g.nbits.rem(), 2);
    let n = g.insert(ns.pop().unwrap(), None);
    g.insert(ns.pop().unwrap(), Some(n));
    assert_eq!(g.nodes.len(), NBITS_GRAN);
    assert_eq!(g.nbits.len(), NBITS_GRAN);
    assert_eq!(g.nbits.rem(), 0);

    g.insert(node(1), None);
    assert_eq!(g.nodes.len(), NBITS_GRAN * 2);
    assert_eq!(g.nbits.len(), NBITS_GRAN * 2);
    assert_eq!(g.nbits.rem(), NBITS_GRAN - 1);
    let n2 = g.insert(node(2), None);
    assert_eq!(g.nodes.len(), NBITS_GRAN * 2);
    assert_eq!(g.nbits.len(), NBITS_GRAN * 2);
    assert_eq!(g.nbits.rem(), NBITS_GRAN - 2);
    g.remove(n);
    assert_eq!(g.nodes.len(), NBITS_GRAN * 2);
    assert_eq!(g.nbits.len(), NBITS_GRAN * 2);
    assert_eq!(g.nbits.rem(), NBITS_GRAN);
    g.remove(n2);
    assert_eq!(g.nodes.len(), NBITS_GRAN * 2);
    assert_eq!(g.nbits.len(), NBITS_GRAN * 2);
    assert_eq!(g.nbits.rem(), NBITS_GRAN + 1);

    let n0 = g.insert(node(3), None);
    let mut n = n0;
    for i in 0..g.nbits.rem() {
        n = g.insert(node(i), Some(n));
    }
    assert_eq!(g.nodes.len(), NBITS_GRAN * 2);
    assert_eq!(g.nbits.len(), NBITS_GRAN * 2);
    assert_eq!(g.nbits.rem(), 0);
    let mut n = g.remove(n);
    assert_eq!(g.nodes.len(), NBITS_GRAN * 2);
    assert_eq!(g.nbits.len(), NBITS_GRAN * 2);
    assert_eq!(g.nbits.rem(), 1);
    let n = g.insert(n.pop().unwrap(), None);
    g.insert(node(6), Some(n));
    assert_eq!(g.nodes.len(), NBITS_GRAN * 4);
    assert_eq!(g.nbits.len(), NBITS_GRAN * 4);
    assert_eq!(g.nbits.rem(), NBITS_GRAN * 2 - 1);

    for i in 0..g.nbits.rem() {
        g.insert(node(i), None);
    }
    assert_eq!(g.nodes.len(), NBITS_GRAN * 4);
    assert_eq!(g.nbits.len(), NBITS_GRAN * 4);
    assert_eq!(g.nbits.rem(), 0);
    g.insert(node(9), Some(n));
    assert_eq!(g.nodes.len(), NBITS_GRAN * 8);
    assert_eq!(g.nbits.len(), NBITS_GRAN * 8);
    assert_eq!(g.nbits.rem(), NBITS_GRAN * 4 - 1);
    g.remove(n0);
    assert_eq!(g.nodes.len(), NBITS_GRAN * 8);
    assert_eq!(g.nbits.len(), NBITS_GRAN * 8);
    assert_eq!(g.nbits.rem(), NBITS_GRAN * 4 + NBITS_GRAN - 1);
}

#[test]
fn update_one() {
    let mut g = Graph::new();

    let m1 = Mat4::from(1.0);
    let n1 = g.insert(Node::Xform(m1), None);
    g.update(n1);
    assert_eq!(g.world(n1), &m1);
    let m2 = Mat4::scale(2.0, 2.0, 2.0);
    *g.local_mut(n1) = m2;
    assert_eq!(g.world(n1), &m1);
    g.update(n1);
    assert_eq!(g.world(n1), &m2);

    let n2 = g.insert(Node::Xform(m2), None);
    g.update(n2);
    assert_eq!(g.world(n2), &m2);
    *g.local_mut(n2) = m1;
    assert_eq!(g.world(n2), &m2);
    g.update(n2);
    assert_eq!(g.world(n2), &m1);
    assert_eq!(g.world(n1), &m2);
    g.update(n1);
    assert_eq!(g.world(n2), &m1);
    assert_eq!(g.world(n1), &m2);

    let n11 = g.insert(
        Node::Light(Light::new_white(LightType::Directional, 1000.0), m1),
        Some(n1),
    );
    g.update(n11);
    assert_eq!(g.world(n11), &m1);
    assert_eq!(g.world(n1), &m2);
    let m3 = Mat4::translation(-10.0, 2.0, -6.0);
    *g.local_mut(n11) = m3;
    assert_eq!(g.world(n11), &m1);
    g.update(n11);
    assert_eq!(g.world(n11), &m3);
    assert_eq!(g.world(n1), &m2);
    assert_eq!(g.world(n2), &m1);
    *g.local_mut(n1) = m1;
    assert_eq!(g.world(n11), &m3);
    assert_eq!(g.world(n1), &m2);
    g.update(n11);
    g.update(n2);
    assert_eq!(g.world(n11), &m3);
    assert_eq!(g.world(n1), &m2);
    assert_eq!(g.world(n2), &m1);
    *g.local_mut(n11) = m1;
    g.update(n11);
    assert_eq!(g.world(n11), &m1);
    assert_eq!(g.world(n1), &m2);
}

#[test]
fn update() {
    let mut g = Graph::new();

    let light = |m| Node::Light(Light::new_white(LightType::Directional, 1000.0), m);
    let xform = |m| Node::Xform(m);

    let m1 = Mat4::scale(2.0, 2.0, 2.0);
    let m2 = Mat4::scale(4.0, 4.0, 4.0);
    let m3 = Mat4::scale(-1.0, -1.0, -1.0);
    let m4 = Mat4::translation(1.0, 2.0, 3.0);
    let m5 = Mat4::translation(-1.0, -2.0, -2.0);
    let m6 = Mat4::translation(30.0, 20.0, 10.0);
    let m7 = Mat4::translation(100.0, 0.0, 0.0);
    let m8 = Mat4::translation(0.0, 100.0, 0.0);
    let m9 = Mat4::translation(0.0, 0.0, 100.0);

    let n1 = g.insert(xform(Mat4::from(0.0)), None);
    let n11 = g.insert(light(m2), Some(n1));
    let n12 = g.insert(xform(m3), Some(n1));
    let n121 = g.insert(xform(m4), Some(n12));
    let n122 = g.insert(light(m5), Some(n12));
    let n123 = g.insert(xform(m6), Some(n12));
    let n1221 = g.insert(xform(m7), Some(n122));
    let n1222 = g.insert(light(m8), Some(n122));
    let n1231 = g.insert(xform(m9), Some(n123));
    let n2 = g.insert(light(Mat4::from(0.0)), None);
    let n21 = g.insert(xform(m8), Some(n2));
    let n22 = g.insert(light(m7), Some(n2));
    let n23 = g.insert(light(m6), Some(n2));
    let n24 = g.insert(xform(m5), Some(n2));
    let n25 = g.insert(xform(m4), Some(n2));
    let n26 = g.insert(light(m3), Some(n2));
    let n3 = g.insert(xform(Mat4::from(0.0)), None);
    let n31 = g.insert(xform(m6), Some(n3));

    let w1 = m1;
    let w11 = w1 * m2;
    let w12 = w1 * m3;
    let w121 = w12 * m4;
    let w122 = w12 * m5;
    let w123 = w12 * m6;
    let w1221 = w122 * m7;
    let w1222 = w122 * m8;
    let w1231 = w123 * m9;
    let w2 = m9;
    let w21 = w2 * m8;
    let w22 = w2 * m7;
    let w23 = w2 * m6;
    let w24 = w2 * m5;
    let w25 = w2 * m4;
    let w26 = w2 * m3;
    let w3 = m3;
    let w31 = w3 * m6;

    *g.local_mut(n1) = m1;
    g.update(n1);
    g.assert_wld(n1);
    assert_eq!(g.world(n1), &w1);
    assert_eq!(g.world(n11), &w11);
    assert_eq!(g.world(n12), &w12);
    assert_eq!(g.world(n121), &w121);
    assert_eq!(g.world(n122), &w122);
    assert_eq!(g.world(n123), &w123);
    assert_eq!(g.world(n1221), &w1221);
    assert_eq!(g.world(n1222), &w1222);
    assert_eq!(g.world(n1231), &w1231);

    *g.local_mut(n2) = m9;
    g.update(n2);
    g.assert_wld(n2);
    assert_eq!(g.world(n2), &w2);
    assert_eq!(g.world(n21), &w21);
    assert_eq!(g.world(n22), &w22);
    assert_eq!(g.world(n23), &w23);
    assert_eq!(g.world(n24), &w24);
    assert_eq!(g.world(n25), &w25);
    assert_eq!(g.world(n26), &w26);
    assert_eq!(g.world(n1), &w1);
    assert_eq!(g.world(n11), &w11);
    assert_eq!(g.world(n12), &w12);
    assert_eq!(g.world(n121), &w121);
    assert_eq!(g.world(n122), &w122);
    assert_eq!(g.world(n123), &w123);
    assert_eq!(g.world(n1221), &w1221);
    assert_eq!(g.world(n1222), &w1222);
    assert_eq!(g.world(n1231), &w1231);

    *g.local_mut(n3) = m3;
    g.update(n3);
    g.assert_wld(n3);
    assert_eq!(g.world(n3), &w3);
    assert_eq!(g.world(n31), &w31);
    assert_eq!(g.world(n2), &w2);
    assert_eq!(g.world(n21), &w21);
    assert_eq!(g.world(n22), &w22);
    assert_eq!(g.world(n23), &w23);
    assert_eq!(g.world(n24), &w24);
    assert_eq!(g.world(n25), &w25);
    assert_eq!(g.world(n26), &w26);
    assert_eq!(g.world(n1), &w1);
    assert_eq!(g.world(n11), &w11);
    assert_eq!(g.world(n12), &w12);
    assert_eq!(g.world(n121), &w121);
    assert_eq!(g.world(n122), &w122);
    assert_eq!(g.world(n123), &w123);
    assert_eq!(g.world(n1221), &w1221);
    assert_eq!(g.world(n1222), &w1222);
    assert_eq!(g.world(n1231), &w1231);

    *g.local_mut(n1) = m9;
    g.update(n1);
    g.assert_wld(n1);

    *g.local_mut(n1) = m5;
    g.update(n11);
    g.assert_wld(n1);

    *g.local_mut(n12) = m8;
    g.update(n11);
    g.assert_wld(n11);
    // Panics.
    //g.assert_wld(n1);

    *g.local_mut(n2) = m4;
    g.update(n2);
    g.assert_wld(n2);
    g.assert_wld(n21);
    g.assert_wld(n22);
    g.assert_wld(n23);
    g.assert_wld(n24);
    g.assert_wld(n25);

    *g.local_mut(n23) = m7;
    g.update(n23);
    g.assert_wld(n22);
    g.assert_wld(n23);
    g.assert_wld(n24);
    g.assert_wld(n25);
    g.assert_wld(n26);
    // Panics.
    //g.assert_wld(n2);

    *g.local_mut(n31) = m6;
    g.update(n31);
    // Panics.
    //g.assert_wld(n3);
    g.assert_wld(n31);
    *g.local_mut(n31) = m3;
    g.update(n3);
    g.assert_wld(n3);
    g.assert_wld(n31);
    *g.local_mut(n31) = m1;
    g.update(n31);
    // Panics.
    //g.assert_wld(n3);
    g.assert_wld(n31);
}

#[test]
fn ignore() {
    let mut g = Graph::new();

    let m1 = Mat4::scale(0.5, 0.5, 0.5);
    let m2 = Mat4::translation(-1.0, -2.0, -3.0);
    let m3 = Mat4::scale(2.0, 2.0, 2.0);
    let m4 = Mat4::translation(16.0, 64.0, 256.0);
    let m5 = Mat4::scale(0.25, 0.25, 0.25);
    let m6 = Mat4::translation(30.0, 20.0, 10.0);

    let n1 = g.insert(
        Node::Light(Light::new_white(LightType::Directional, 600.0), m1),
        None,
    );
    let n11 = g.insert(Node::Xform(m2), Some(n1));
    let n12 = g.insert(
        Node::Light(Light::new_white(LightType::Directional, 500.0), m3),
        Some(n1),
    );
    let n121 = g.insert(Node::Xform(m4), Some(n12));

    let n2 = g.insert(Node::Xform(m2), None);

    let n3 = g.insert(
        Node::Light(Light::new_white(LightType::Directional, 300.0), m3),
        None,
    );
    let n31 = g.insert(Node::Xform(m4), Some(n3));
    let n311 = g.insert(Node::Xform(m5), Some(n31));
    let n3111 = g.insert(Node::Xform(m6), Some(n311));

    let eq_chain = |w: &Mat4<f32>, ms: &[Mat4<f32>]| -> bool {
        let mut world = Mat4::from(1f32);
        for i in ms {
            world *= i;
        }
        w == &world
    };

    g.update(n1);
    assert!(eq_chain(g.world(n1), &[m1]));
    assert!(eq_chain(g.world(n11), &[m1, m2]));
    assert!(eq_chain(g.world(n12), &[m1, m3]));
    assert!(eq_chain(g.world(n121), &[m1, m3, m4]));
    assert!(!eq_chain(g.world(n2), &[m2]));
    assert!(!eq_chain(g.world(n3), &[m3]));
    assert!(!eq_chain(g.world(n31), &[m3, m4]));
    assert!(!eq_chain(g.world(n311), &[m3, m4, m5]));
    assert!(!eq_chain(g.world(n3111), &[m3, m4, m5, m6]));

    g.update(n2);
    assert!(eq_chain(g.world(n1), &[m1]));
    assert!(eq_chain(g.world(n11), &[m1, m2]));
    assert!(eq_chain(g.world(n12), &[m1, m3]));
    assert!(eq_chain(g.world(n121), &[m1, m3, m4]));
    assert!(eq_chain(g.world(n2), &[m2]));
    assert!(!eq_chain(g.world(n3), &[m3]));
    assert!(!eq_chain(g.world(n31), &[m3, m4]));
    assert!(!eq_chain(g.world(n311), &[m3, m4, m5]));
    assert!(!eq_chain(g.world(n3111), &[m3, m4, m5, m6]));

    g.update(n3);
    assert!(eq_chain(g.world(n1), &[m1]));
    assert!(eq_chain(g.world(n11), &[m1, m2]));
    assert!(eq_chain(g.world(n12), &[m1, m3]));
    assert!(eq_chain(g.world(n121), &[m1, m3, m4]));
    assert!(eq_chain(g.world(n2), &[m2]));
    assert!(eq_chain(g.world(n3), &[m3]));
    assert!(eq_chain(g.world(n31), &[m3, m4]));
    assert!(eq_chain(g.world(n311), &[m3, m4, m5]));
    assert!(eq_chain(g.world(n3111), &[m3, m4, m5, m6]));

    g.ignore(n1, true);
    g.update(n1);
    assert!(eq_chain(g.world(n1), &[m1]));
    assert!(eq_chain(g.world(n11), &[m1, m2]));
    assert!(eq_chain(g.world(n12), &[m1, m3]));
    assert!(eq_chain(g.world(n121), &[m1, m3, m4]));
    assert!(eq_chain(g.world(n2), &[m2]));
    assert!(eq_chain(g.world(n3), &[m3]));
    assert!(eq_chain(g.world(n31), &[m3, m4]));
    assert!(eq_chain(g.world(n311), &[m3, m4, m5]));
    assert!(eq_chain(g.world(n3111), &[m3, m4, m5, m6]));

    *g.local_mut(n1) = m6;
    g.update(n1);
    assert!(eq_chain(g.world(n1), &[m1]));
    assert!(eq_chain(g.world(n11), &[m1, m2]));
    assert!(eq_chain(g.world(n12), &[m1, m3]));
    assert!(eq_chain(g.world(n121), &[m1, m3, m4]));
    assert!(eq_chain(g.world(n2), &[m2]));
    assert!(eq_chain(g.world(n3), &[m3]));
    assert!(eq_chain(g.world(n31), &[m3, m4]));
    assert!(eq_chain(g.world(n311), &[m3, m4, m5]));
    assert!(eq_chain(g.world(n3111), &[m3, m4, m5, m6]));

    g.ignore(n1, false);
    g.update(n1);
    assert!(eq_chain(g.world(n1), &[m6]));
    assert!(eq_chain(g.world(n11), &[m6, m2]));
    assert!(eq_chain(g.world(n12), &[m6, m3]));
    assert!(eq_chain(g.world(n121), &[m6, m3, m4]));
    assert!(eq_chain(g.world(n2), &[m2]));
    assert!(eq_chain(g.world(n3), &[m3]));
    assert!(eq_chain(g.world(n31), &[m3, m4]));
    assert!(eq_chain(g.world(n311), &[m3, m4, m5]));
    assert!(eq_chain(g.world(n3111), &[m3, m4, m5, m6]));

    g.ignore(n11, true);
    *g.local_mut(n11) = m1;
    g.update(n1);
    assert!(eq_chain(g.world(n1), &[m6]));
    assert!(eq_chain(g.world(n11), &[m6, m2]));
    assert!(eq_chain(g.world(n12), &[m6, m3]));
    assert!(eq_chain(g.world(n121), &[m6, m3, m4]));
    assert!(eq_chain(g.world(n2), &[m2]));
    assert!(eq_chain(g.world(n3), &[m3]));
    assert!(eq_chain(g.world(n31), &[m3, m4]));
    assert!(eq_chain(g.world(n311), &[m3, m4, m5]));
    assert!(eq_chain(g.world(n3111), &[m3, m4, m5, m6]));

    g.ignore(n11, false);
    g.update(n1);
    assert!(eq_chain(g.world(n1), &[m6]));
    assert!(eq_chain(g.world(n11), &[m6, m1]));
    assert!(eq_chain(g.world(n12), &[m6, m3]));
    assert!(eq_chain(g.world(n121), &[m6, m3, m4]));
    assert!(eq_chain(g.world(n2), &[m2]));
    assert!(eq_chain(g.world(n3), &[m3]));
    assert!(eq_chain(g.world(n31), &[m3, m4]));
    assert!(eq_chain(g.world(n311), &[m3, m4, m5]));
    assert!(eq_chain(g.world(n3111), &[m3, m4, m5, m6]));

    g.ignore(n12, true);
    *g.local_mut(n12) = m2;
    g.update(n1);
    assert!(eq_chain(g.world(n1), &[m6]));
    assert!(eq_chain(g.world(n11), &[m6, m1]));
    assert!(eq_chain(g.world(n12), &[m6, m3]));
    assert!(eq_chain(g.world(n121), &[m6, m3, m4]));
    assert!(eq_chain(g.world(n2), &[m2]));
    assert!(eq_chain(g.world(n3), &[m3]));
    assert!(eq_chain(g.world(n31), &[m3, m4]));
    assert!(eq_chain(g.world(n311), &[m3, m4, m5]));
    assert!(eq_chain(g.world(n3111), &[m3, m4, m5, m6]));

    g.ignore(n12, false);
    g.update(n1);
    assert!(eq_chain(g.world(n1), &[m6]));
    assert!(eq_chain(g.world(n11), &[m6, m1]));
    assert!(eq_chain(g.world(n12), &[m6, m2]));
    assert!(eq_chain(g.world(n121), &[m6, m2, m4]));
    assert!(eq_chain(g.world(n2), &[m2]));
    assert!(eq_chain(g.world(n3), &[m3]));
    assert!(eq_chain(g.world(n31), &[m3, m4]));
    assert!(eq_chain(g.world(n311), &[m3, m4, m5]));
    assert!(eq_chain(g.world(n3111), &[m3, m4, m5, m6]));

    g.ignore(n3111, true);
    g.ignore(n2, true);
    *g.local_mut(n31) = m2;
    *g.local_mut(n2) = m1;
    g.update(n3);
    g.update(n2);
    assert!(eq_chain(g.world(n1), &[m6]));
    assert!(eq_chain(g.world(n11), &[m6, m1]));
    assert!(eq_chain(g.world(n12), &[m6, m2]));
    assert!(eq_chain(g.world(n121), &[m6, m2, m4]));
    assert!(eq_chain(g.world(n2), &[m2]));
    assert!(eq_chain(g.world(n3), &[m3]));
    assert!(eq_chain(g.world(n31), &[m3, m2]));
    assert!(eq_chain(g.world(n311), &[m3, m2, m5]));
    assert!(eq_chain(g.world(n3111), &[m3, m4, m5, m6]));

    g.ignore(n2, false);
    g.ignore(n3111, false);
    g.update(n2);
    g.update(n3);
    assert!(eq_chain(g.world(n1), &[m6]));
    assert!(eq_chain(g.world(n11), &[m6, m1]));
    assert!(eq_chain(g.world(n12), &[m6, m2]));
    assert!(eq_chain(g.world(n121), &[m6, m2, m4]));
    assert!(eq_chain(g.world(n2), &[m1]));
    assert!(eq_chain(g.world(n3), &[m3]));
    assert!(eq_chain(g.world(n31), &[m3, m2]));
    assert!(eq_chain(g.world(n311), &[m3, m2, m5]));
    assert!(eq_chain(g.world(n3111), &[m3, m2, m5, m6]));

    g.ignore(n3, true);
    *g.local_mut(n3) = m5;
    *g.local_mut(n311) = m4;
    g.update(n31);
    assert!(eq_chain(g.world(n1), &[m6]));
    assert!(eq_chain(g.world(n11), &[m6, m1]));
    assert!(eq_chain(g.world(n12), &[m6, m2]));
    assert!(eq_chain(g.world(n121), &[m6, m2, m4]));
    assert!(eq_chain(g.world(n2), &[m1]));
    assert!(eq_chain(g.world(n3), &[m3]));
    assert!(eq_chain(g.world(n31), &[m3, m2]));
    assert!(eq_chain(g.world(n311), &[m3, m2, m4]));
    assert!(eq_chain(g.world(n3111), &[m3, m2, m4, m6]));

    g.ignore(n3, false);
    g.ignore(n311, true);
    g.update(n3);
    assert!(eq_chain(g.world(n1), &[m6]));
    assert!(eq_chain(g.world(n11), &[m6, m1]));
    assert!(eq_chain(g.world(n12), &[m6, m2]));
    assert!(eq_chain(g.world(n121), &[m6, m2, m4]));
    assert!(eq_chain(g.world(n2), &[m1]));
    assert!(eq_chain(g.world(n3), &[m5]));
    assert!(eq_chain(g.world(n31), &[m5, m2]));
    assert!(eq_chain(g.world(n311), &[m3, m2, m4]));
    assert!(eq_chain(g.world(n3111), &[m3, m2, m4, m6]));

    g.ignore(n311, false);
    g.update(n3);
    assert!(eq_chain(g.world(n1), &[m6]));
    assert!(eq_chain(g.world(n11), &[m6, m1]));
    assert!(eq_chain(g.world(n12), &[m6, m2]));
    assert!(eq_chain(g.world(n121), &[m6, m2, m4]));
    assert!(eq_chain(g.world(n2), &[m1]));
    assert!(eq_chain(g.world(n3), &[m5]));
    assert!(eq_chain(g.world(n31), &[m5, m2]));
    assert!(eq_chain(g.world(n311), &[m5, m2, m4]));
    assert!(eq_chain(g.world(n3111), &[m5, m2, m4, m6]));
}
