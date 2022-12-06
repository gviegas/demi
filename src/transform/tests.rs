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
