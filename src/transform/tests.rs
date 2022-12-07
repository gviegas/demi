// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use crate::linear::Mat4;
use crate::transform::Transform;

#[test]
fn new() {
    let m = Mat4::new(&[[1.0; 4], [2.0; 4], [-1.0; 4], [0.5; 4]]);
    let mut graph = Transform::new(&m);
    assert!(graph.len() == 1);
    let id = graph.id();
    let local = graph.local(&id);
    for i in 0..4 {
        for j in 0..4 {
            assert_eq!(local[i][j], m[i][j]);
        }
    }
    let local_mut = graph.local_mut(&id);
    *local_mut = m.clone() * Mat4::scale(2.0, 2.0, 2.0);
    for i in 0..3 {
        for j in 0..3 {
            assert_eq!(local_mut[i][j], m[i][j] * 2.0);
        }
    }
}

#[test]
fn insert() {
    let mut graph = Transform::new(&Mat4::from(1.0));
    assert_eq!(graph.len(), 1);
    let xa = graph.insert(&graph.id(), &Mat4::translation(0.0, 0.0, -10.0));
    assert_eq!(graph.len(), 2);
    let xaa = graph.insert(&xa, &Mat4::rotation_y(std::f32::consts::FRAC_PI_2));
    assert_eq!(graph.len(), 3);
    let xb = graph.insert(&graph.id(), &Mat4::scale(-1.0, -1.0, -1.0));
    assert_eq!(graph.len(), 4);

    // Note:
    // - front insertions
    // - first_child.prev == parent

    let node = graph.nodes[graph.id().0].as_ref().unwrap();
    assert_eq!(node.prev, None);
    assert_eq!(node.next, None);
    assert_eq!(node.sub, Some(xb.0));
    assert_eq!(node.data, 0);

    let node = graph.nodes[xa.0].as_ref().unwrap();
    assert_eq!(node.prev, Some(xb.0));
    assert_eq!(node.next, None);
    assert_eq!(node.sub, Some(xaa.0));
    assert_eq!(node.data, 1);

    let node = graph.nodes[xaa.0].as_ref().unwrap();
    assert_eq!(node.prev, Some(xa.0));
    assert_eq!(node.next, None);
    assert_eq!(node.sub, None);
    assert_eq!(node.data, 2);

    let node = graph.nodes[xb.0].as_ref().unwrap();
    assert_eq!(node.prev, Some(graph.id().0));
    assert_eq!(node.next, Some(xa.0));
    assert_eq!(node.sub, None);
    assert_eq!(node.data, 3);

    assert_eq!(graph.data[0].node, 0);
    assert_eq!(graph.data[1].node, 1);
    assert_eq!(graph.data[2].node, 2);
    assert_eq!(graph.data[3].node, 3);
}
