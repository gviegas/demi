// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use crate::light::{Light, LightType};
use crate::linear::Mat4;
use crate::scene::{Node, NodeType, Scene};

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

#[test]
fn insert() {
    let mut scene = Scene::default();
    assert!(scene.nodes.is_empty());
    assert!(scene.drawables.is_empty());
    assert!(scene.lights.is_empty());
    assert!(scene.xforms.is_empty());

    // TODO: Drawable.
    let light = Light::new_white(LightType::Directional, 1e6);
    let xform = Mat4::translation(2.0, 1.0, -5.0);

    let nd0 = scene.insert(None, Node::Light(light, Mat4::rotation_x(0.7854)));
    assert_eq!(NodeType::Light, nd0.node_type);
    assert_eq!(0, nd0.node_idx);
    assert_eq!(2, scene.graph.len());
    assert_eq!(1, scene.nodes.len());
    assert!(scene.drawables.is_empty());
    assert_eq!(1, scene.lights.len());
    assert!(scene.xforms.is_empty());

    let nd1 = scene.insert(None, Node::Xform(xform));
    assert_eq!(NodeType::Xform, nd1.node_type);
    assert_eq!(1, nd1.node_idx);
    assert_eq!(3, scene.graph.len());
    assert_eq!(2, scene.nodes.len());
    assert!(scene.drawables.is_empty());
    assert_eq!(1, scene.lights.len());
    assert_eq!(1, scene.xforms.len());

    let nd2 = scene.insert(
        Some(&nd0),
        Node::Xform(xform * Mat4::scale(-1.0, -1.0, -1.0)),
    );
    assert_eq!(NodeType::Xform, nd2.node_type);
    assert_eq!(2, nd2.node_idx);
    assert_eq!(4, scene.graph.len());
    assert_eq!(3, scene.nodes.len());
    assert!(scene.drawables.is_empty());
    assert_eq!(1, scene.lights.len());
    assert_eq!(2, scene.xforms.len());

    let nd3 = scene.insert(
        Some(&nd2),
        Node::Light(
            Light::new_white(LightType::Point { range: 6.5 }, 500.0),
            Mat4::translation(0.0, 15.0, 0.0),
        ),
    );
    assert_eq!(NodeType::Light, nd3.node_type);
    assert_eq!(3, nd3.node_idx);
    assert_eq!(5, scene.graph.len());
    assert_eq!(4, scene.nodes.len());
    assert!(scene.drawables.is_empty());
    assert_eq!(2, scene.lights.len());
    assert_eq!(2, scene.xforms.len());

    assert_eq!(0, scene.nodes[nd0.node_idx].unwrap()); // Into `scene.lights`.
    assert_eq!(0, scene.nodes[nd1.node_idx].unwrap()); // Into `scene.xforms`.
    assert_eq!(1, scene.nodes[nd2.node_idx].unwrap()); // Into `scene.xforms`.
    assert_eq!(1, scene.nodes[nd3.node_idx].unwrap()); // Into `scene.lights`.
}

#[test]
fn remove() {
    let mut scene = Scene::default();

    // TODO: Drawable.
    let light = Light::new_white(LightType::Directional, 1e6);
    let xform = Mat4::translation(2.0, 1.0, -5.0);

    let nd0 = scene.insert(None, Node::Light(light, Mat4::rotation_x(0.7854)));
    assert_eq!(2, scene.graph.len());
    assert_eq!(1, scene.nodes.len());
    assert!(scene.drawables.is_empty());
    assert_eq!(1, scene.lights.len());
    assert!(scene.xforms.is_empty());
    let light = match scene.remove(nd0) {
        Node::Light(l, _) => {
            assert_eq!(1, scene.graph.len());
            // Should keep node entry as `None` (vacant).
            assert_eq!(1, scene.nodes.len());
            assert!(scene.nodes[0].is_none());
            assert!(scene.drawables.is_empty());
            assert!(scene.lights.is_empty());
            assert!(scene.xforms.is_empty());
            l
        }
        x => panic!("unexpected Node: {:#?}", x),
    };

    let nd0 = scene.insert(None, Node::Light(light, Mat4::rotation_x(0.7854)));
    assert_eq!(2, scene.graph.len());
    // Should insert in the vacant node.
    assert_eq!(1, scene.nodes.len());
    assert_eq!(1, scene.lights.len());
    let nd1 = scene.insert(None, Node::Xform(xform));
    let nd2 = scene.insert(
        Some(&nd0),
        Node::Xform(xform * Mat4::scale(-1.0, -1.0, -1.0)),
    );
    let nd3 = scene.insert(
        Some(&nd2),
        Node::Light(
            Light::new_white(LightType::Point { range: 6.5 }, 500.0),
            Mat4::translation(0.0, 15.0, 0.0),
        ),
    );
    assert_eq!(5, scene.graph.len());
    assert_eq!(4, scene.nodes.len());
    assert!(scene.drawables.is_empty());
    assert_eq!(2, scene.lights.len());
    assert_eq!(2, scene.xforms.len());
    assert_eq!(0, scene.nodes[nd0.node_idx].unwrap()); // Into `scene.lights`.
    assert_eq!(0, scene.nodes[nd1.node_idx].unwrap()); // Into `scene.xforms`.
    assert_eq!(1, scene.nodes[nd2.node_idx].unwrap()); // Into `scene.xforms`.
    assert_eq!(1, scene.nodes[nd3.node_idx].unwrap()); // Into `scene.lights`.

    match scene.remove(nd1) {
        Node::Xform(_) => {
            assert_eq!(4, scene.graph.len());
            assert_eq!(4, scene.nodes.len());
            assert_eq!(2, scene.lights.len());
            assert_eq!(1, scene.xforms.len());
            assert_eq!(0, scene.nodes[nd0.node_idx].unwrap()); // Into `scene.lights`.
                                                               // Should swap-remove.
            assert_eq!(0, scene.nodes[nd2.node_idx].unwrap()); // Into `scene.xforms`.
            assert_eq!(1, scene.nodes[nd3.node_idx].unwrap()); // Into `scene.lights`.
        }
        x => panic!("unexpected Node: {:#?}", x),
    };

    let nd3_idx = nd3.node_idx;
    match scene.remove(nd3) {
        Node::Light(..) => {
            assert_eq!(3, scene.graph.len());
            assert_eq!(4, scene.nodes.len());
            assert!(scene.nodes[nd3_idx].is_none());
            assert_eq!(1, scene.lights.len());
            assert_eq!(1, scene.xforms.len());
            assert_eq!(0, scene.nodes[nd0.node_idx].unwrap()); // Into `scene.lights`.
            assert_eq!(0, scene.nodes[nd2.node_idx].unwrap()); // Into `scene.xforms`.
        }
        x => panic!("unexpected Node: {:#?}", x),
    };

    let node_idx = scene.node_idx;
    let none_cnt = scene.none_cnt;
    assert_eq!(node_idx, nd3_idx);
    assert_eq!(
        none_cnt,
        scene.nodes.len() - scene.drawables.len() - scene.lights.len() - scene.xforms.len()
    );

    let nd2_idx = nd2.node_idx;
    let xform = match scene.remove(nd2) {
        Node::Xform(x) => {
            assert_eq!(2, scene.graph.len());
            assert_eq!(4, scene.nodes.len());
            assert!(scene.nodes[nd2_idx].is_none());
            assert_eq!(1, scene.lights.len());
            assert!(scene.xforms.is_empty());
            assert_eq!(0, scene.nodes[nd0.node_idx].unwrap()); // Into `scene.lights`.
            x
        }
        x => panic!("unexpected Node: {:#?}", x),
    };

    let node_idx = scene.node_idx;
    let none_cnt = scene.none_cnt;
    assert_eq!(node_idx, nd2_idx);
    assert_eq!(
        none_cnt,
        scene.nodes.len() - scene.drawables.len() - scene.lights.len() - scene.xforms.len()
    );

    let nd4 = scene.insert(Some(&nd0), Node::Xform(xform));
    // Should insert into `scene.nodes[node_idx]` (latest vacancy).
    assert_eq!(node_idx, nd4.node_idx);
    assert_eq!(none_cnt - 1, scene.none_cnt);
    assert_eq!(3, scene.graph.len());
    assert_eq!(4, scene.nodes.len());
    assert_eq!(1, scene.lights.len());
    assert_eq!(1, scene.xforms.len());
    assert_eq!(0, scene.nodes[nd0.node_idx].unwrap()); // Into `scene.lights`.
    assert_eq!(0, scene.nodes[nd4.node_idx].unwrap()); // Into `scene.xforms`.
}
