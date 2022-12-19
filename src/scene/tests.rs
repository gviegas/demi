// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use crate::scene::Scene;

#[test]
fn scene_graph() {
    let scene = Scene::default();
    let graph = scene.graph();
    // The root transform is not accessible as a scene node
    // and should be the identity.
    let local = graph.local(&graph.id());
    for i in 0..4 {
        assert_eq!(1.0, local[i][i]);
        for j in i + 1..4 {
            assert_eq!(local[i][j], local[j][i]);
            assert_eq!(0.0, local[i][j]);
        }
    }
    assert_eq!(1, graph.len());
}
