// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use crate::light::{Light, LightType};
use crate::linear::{Mat4, Vec3};
use crate::scene::{Node, NodeType, Scene};

#[test]
fn scene_graph() {
    let scene = Scene::default();
    let graph = scene.graph();
    // The root transform is not accessible as a scene node
    // and should be the identity.
    let local = graph.local(graph.id());
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

    let nd0 = scene.insert(Node::Light(light, Mat4::rotation_x(0.7854)), None);
    assert_eq!(NodeType::Light, nd0.node_type);
    assert_eq!(0, nd0.node_idx);
    assert_eq!(2, scene.graph.len());
    assert_eq!(1, scene.nodes.len());
    assert!(scene.drawables.is_empty());
    assert_eq!(1, scene.lights.len());
    assert!(scene.xforms.is_empty());

    let nd1 = scene.insert(Node::Xform(xform), None);
    assert_eq!(NodeType::Xform, nd1.node_type);
    assert_eq!(1, nd1.node_idx);
    assert_eq!(3, scene.graph.len());
    assert_eq!(2, scene.nodes.len());
    assert!(scene.drawables.is_empty());
    assert_eq!(1, scene.lights.len());
    assert_eq!(1, scene.xforms.len());

    let nd2 = scene.insert(
        Node::Xform(xform * Mat4::scale(-1.0, -1.0, -1.0)),
        Some(nd0),
    );
    assert_eq!(NodeType::Xform, nd2.node_type);
    assert_eq!(2, nd2.node_idx);
    assert_eq!(4, scene.graph.len());
    assert_eq!(3, scene.nodes.len());
    assert!(scene.drawables.is_empty());
    assert_eq!(1, scene.lights.len());
    assert_eq!(2, scene.xforms.len());

    let nd3 = scene.insert(
        Node::Light(
            Light::new_white(LightType::Point { range: 6.5 }, 500.0),
            Mat4::translation(0.0, 15.0, 0.0),
        ),
        Some(nd2),
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

    let nd0 = scene.insert(Node::Light(light, Mat4::rotation_x(0.7854)), None);
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

    let nd0 = scene.insert(Node::Light(light, Mat4::rotation_x(0.7854)), None);
    assert_eq!(2, scene.graph.len());
    // Should insert in the vacant node.
    assert_eq!(1, scene.nodes.len());
    assert_eq!(1, scene.lights.len());
    let nd1 = scene.insert(Node::Xform(xform), None);
    let nd2 = scene.insert(
        Node::Xform(xform * Mat4::scale(-1.0, -1.0, -1.0)),
        Some(nd0),
    );
    let nd3 = scene.insert(
        Node::Light(
            Light::new_white(LightType::Point { range: 6.5 }, 500.0),
            Mat4::translation(0.0, 15.0, 0.0),
        ),
        Some(nd2),
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

    let nd4 = scene.insert(Node::Xform(xform), Some(nd0));
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

#[test]
#[should_panic = "Not a Drawable node: NodeId { node_type: Light, node_idx: 1 }"]
fn not_drawable_node() {
    let mut scene = Scene::default();
    let nd0 = scene.insert(Node::Xform(Mat4::from(1.0)), None);
    let nd1 = scene.insert(
        Node::Light(
            Light::new_white(LightType::Point { range: 4.5 }, 300.0),
            Mat4::translation(0.5, 3.0, -2.0),
        ),
        Some(nd0),
    );
    let _ = scene.drawable(nd1);
}

#[test]
#[should_panic = "Not a Light node: NodeId { node_type: Xform, node_idx: 0 }"]
fn not_light_node() {
    let mut scene = Scene::default();
    let nd0 = scene.insert(Node::Xform(Mat4::from(1.0)), None);
    let nd1 = scene.insert(
        Node::Light(
            Light::new_white(LightType::Point { range: 4.5 }, 300.0),
            Mat4::translation(0.5, 3.0, -2.0),
        ),
        None,
    );
    let _ = scene.light(nd1); // OK.
    let _ = scene.light(nd0);
}

#[test]
fn node_access() {
    // TODO: Drawable.
    let light0 = Light::new(LightType::Directional, 1000.0, Vec3::new(0.9, 0.9, 0.5));
    let light1 = Light::new_white(LightType::Point { range: 4.5 }, 300.0);
    let m0 = Mat4::from(-1.0);
    let m1 = Mat4::translation(0.0, 16.0, 0.0);

    let mut scene = Scene::default();
    let nd0 = scene.insert(Node::Light(light0, m0), None);
    let nd1 = scene.insert(Node::Light(light1, m1), None);

    let ref0 = (scene.light(nd0), scene.local(nd0));
    let ref1 = (scene.light(nd1), scene.local(nd1));
    assert_eq!(1000.0, ref0.0.intensity());
    assert_eq!(300.0, ref1.0.intensity());
    assert_eq!(-1.0, ref0.1[0][0]);
    assert_eq!(16.0, ref1.1[3][1]);

    let mut0 = scene.light_mut(nd0);
    assert_eq!(1000.0, mut0.intensity());
    let mut0 = scene.local_mut(nd0);
    assert_eq!(-1.0, mut0[0][0]);
    let mut1 = scene.light_mut(nd1);
    assert_eq!(300.0, mut1.intensity());
    let mut1 = scene.local_mut(nd1);
    assert_eq!(16.0, mut1[3][1]);
}
