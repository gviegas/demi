// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use crate::linear::Mat4;
use crate::transform::Transform;

#[test]
fn new() {
    let m = Mat4::new([1.0; 4], [2.0; 4], [-1.0; 4], [0.5; 4]);
    let mut graph = Transform::new(m);
    assert!(graph.len() == 1);
    let id = graph.id();
    let local = graph.local(id);
    for i in 0..4 {
        for j in 0..4 {
            assert_eq!(local[i][j], m[i][j]);
        }
    }
    let local_mut = graph.local_mut(id);
    *local_mut = m * Mat4::scale(2.0, 2.0, 2.0);
    for i in 0..3 {
        for j in 0..3 {
            assert_eq!(local_mut[i][j], m[i][j] * 2.0);
        }
    }
}

#[test]
fn insert() {
    // AKA `Transform::default()`.
    let mut graph = Transform::new(Mat4::from(1.0));
    assert_eq!(graph.len(), 1);
    assert_eq!(graph.node_bits.rem(), 31);
    let xa = graph.insert(Mat4::translation(0.0, 0.0, -10.0), graph.id());
    assert_eq!(graph.len(), 2);
    assert_eq!(graph.node_bits.rem(), 30);
    let xaa = graph.insert(Mat4::rotation_y(std::f32::consts::FRAC_PI_2), xa);
    assert_eq!(graph.len(), 3);
    assert_eq!(graph.node_bits.rem(), 29);
    let xb = graph.insert(Mat4::scale(-1.0, -1.0, -1.0), graph.id());
    assert_eq!(graph.len(), 4);
    assert_eq!(graph.node_bits.rem(), 28);

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

    while graph.len() < 32 {
        graph.insert(Default::default(), graph.id());
        assert_eq!(graph.node_bits.len(), 32);
        assert_eq!(graph.node_bits.rem(), 32 - graph.len());
    }
    graph.insert(Default::default(), graph.id());
    assert_eq!(graph.len(), 33);
    assert_eq!(graph.node_bits.len(), 64);
    assert_eq!(graph.node_bits.rem(), 31);
    graph.insert(Default::default(), graph.id());
    assert_eq!(graph.len(), 34);
    assert_eq!(graph.node_bits.len(), 64);
    assert_eq!(graph.node_bits.rem(), 30);
}

#[test]
#[should_panic = "cannot remove root transform"]
fn remove_root() {
    let mut graph = Transform::default();
    graph.remove(graph.id());
}

#[test]
fn remove() {
    let eq_mat = |m, n| {
        let m: Mat4<f32> = m - n;
        for i in 0..4 {
            for j in 0..4 {
                assert!(m[i][j].abs() <= f32::EPSILON);
            }
        }
    };

    let m = Mat4::from(1.0);
    let ma = Mat4::from(2.0);
    let maa = Mat4::from(3.0);
    let mb = Mat4::from(4.0);

    let mut graph = Transform::new(m);
    let xa = graph.insert(ma, graph.id());
    let xaa = graph.insert(maa, xa);
    let xb = graph.insert(mb, graph.id());

    // Note:
    // - `nodes` are set to `None` (vacant) on removal
    // - `data` are swap-removed from `Vec`

    let xa_i = xa.0;
    let xaa_i = xaa.0;
    let xb_i = xb.0;

    assert_eq!(graph.nodes.len(), 32);
    assert_eq!(graph.node_bits.rem(), 28);
    assert_eq!(graph.data.len(), 4);
    eq_mat(graph.remove(xaa), maa);
    assert_eq!(graph.nodes.len(), 32);
    assert_eq!(graph.node_bits.rem(), 29);
    assert_eq!(graph.data.len(), 3);
    assert_eq!(graph.data.last().unwrap().node, xb_i);

    let xaa = graph.insert(maa, xa);
    assert_eq!(xaa.0, xaa_i);
    assert_eq!(graph.nodes.len(), 32);
    assert_eq!(graph.node_bits.rem(), 28);
    assert_eq!(graph.data.len(), 4);
    assert_eq!(graph.data.last().unwrap().node, xaa_i);
    eq_mat(graph.remove(xaa), maa);
    assert_eq!(graph.nodes.len(), 32);
    assert_eq!(graph.node_bits.rem(), 29);
    assert_eq!(graph.data.len(), 3);
    assert_eq!(graph.data.last().unwrap().node, xb_i);

    eq_mat(graph.remove(xa), ma);
    assert_eq!(graph.nodes.len(), 32);
    assert_eq!(graph.node_bits.rem(), 30);
    assert_eq!(graph.data.len(), 2);
    assert_eq!(graph.data.last().unwrap().node, xb_i);

    eq_mat(graph.remove(xb), mb);
    assert_eq!(graph.nodes.len(), 32);
    assert_eq!(graph.node_bits.rem(), 31);
    assert_eq!(graph.data.len(), 1);
    assert_eq!(graph.data.last().unwrap().node, graph.id().0);

    let xb = graph.insert(mb, graph.id());
    let xb_i = xa_i;
    assert_eq!(xb.0, xb_i);
    assert_eq!(graph.nodes.len(), 32);
    assert_eq!(graph.node_bits.rem(), 30);
    assert_eq!(graph.data.len(), 2);
    assert_eq!(graph.data.last().unwrap().node, xb_i);

    let x = graph.insert(Default::default(), graph.id());
    assert_eq!(graph.nodes.len(), 32);
    assert_eq!(graph.node_bits.rem(), 29);
    assert_eq!(graph.data.len(), 3);
    assert_eq!(graph.data[0].node, graph.id().0);
    assert_eq!(graph.nodes[graph.id().0].as_ref().unwrap().data, 0);
    assert_eq!(graph.data[1].node, xb_i);
    assert_eq!(graph.nodes[xb_i].as_ref().unwrap().data, 1);
    assert_eq!(graph.data[2].node, x.0);
    assert_eq!(graph.nodes[x.0].as_ref().unwrap().data, 2);
}

#[test]
fn update_world() {
    let eq_mat = |m, n| {
        let m: Mat4<f32> = m - n;
        for i in 0..4 {
            for j in 0..4 {
                assert!(m[i][j].abs() <= f32::EPSILON);
            }
        }
    };

    let dfl = Mat4::<f32>::default();
    let m = Mat4::from(2.0);
    let ma = Mat4::from(0.25);
    let mb = Mat4::from(3.0);
    let maa = Mat4::from(2.5);

    let mut graph = Transform::new(m);
    let x = graph.id();
    let xa = graph.insert(ma, graph.id());
    let xb = graph.insert(mb, graph.id());
    eq_mat(*graph.world(x), m);
    eq_mat(*graph.world(xa), dfl);
    eq_mat(*graph.world(xb), dfl);

    graph.update_world();
    eq_mat(*graph.world(x), m);
    eq_mat(*graph.world(xa), graph.world(x) * graph.local(xa));
    eq_mat(*graph.world(xb), graph.world(x) * graph.local(xb));

    let xaa = graph.insert(maa, xa);
    eq_mat(*graph.world(x), m);
    eq_mat(*graph.world(xa), graph.world(x) * graph.local(xa));
    eq_mat(*graph.world(xb), graph.world(x) * graph.local(xb));
    eq_mat(*graph.world(xaa), dfl);

    graph.update_world();
    eq_mat(*graph.world(x), m);
    eq_mat(*graph.world(xa), graph.world(x) * graph.local(xa));
    eq_mat(*graph.world(xb), graph.world(x) * graph.local(xb));
    eq_mat(*graph.world(xaa), graph.world(xa) * graph.local(xaa));

    let m = Mat4::translation(-10.0, 0.0, 10.0);
    *graph.local_mut(x) = m;
    graph.update_world();
    eq_mat(*graph.world(x), m);
    eq_mat(*graph.world(xa), graph.world(x) * graph.local(xa));
    eq_mat(*graph.world(xb), graph.world(x) * graph.local(xb));
    eq_mat(*graph.world(xaa), graph.world(xa) * graph.local(xaa));

    *graph.local_mut(xa) = Mat4::translation(2.0, -20.0, -2.0);
    graph.update_world();
    eq_mat(*graph.world(x), m);
    eq_mat(*graph.world(xa), graph.world(x) * graph.local(xa));
    eq_mat(*graph.world(xb), graph.world(x) * graph.local(xb));
    eq_mat(*graph.world(xaa), graph.world(xa) * graph.local(xaa));
}

#[test]
fn changed() {
    let mut graph = Transform::default();
    assert!(!graph.changed(graph.id()));

    let xa = graph.insert(Mat4::from(1.0), graph.id());
    assert!(!graph.changed(graph.id()));
    assert!(graph.changed(xa));

    graph.update_world();
    assert!(!graph.changed(graph.id()));
    assert!(!graph.changed(xa));

    let xb = graph.insert(Mat4::from(1.0), graph.id());
    assert!(!graph.changed(graph.id()));
    assert!(!graph.changed(xa));
    assert!(graph.changed(xb));

    let xaa = graph.insert(Mat4::from(1.0), graph.id());
    assert!(!graph.changed(graph.id()));
    assert!(!graph.changed(xa));
    assert!(graph.changed(xb));
    assert!(graph.changed(xaa));

    *graph.local_mut(xa) *= Mat4::scale(-1.0, -1.0, -1.0);
    assert!(!graph.changed(graph.id()));
    assert!(graph.changed(xa));
    assert!(graph.changed(xb));
    assert!(graph.changed(xaa));

    graph.update_world();
    assert!(!graph.changed(graph.id()));
    assert!(!graph.changed(xa));
    assert!(!graph.changed(xb));
    assert!(!graph.changed(xaa));

    *graph.local_mut(graph.id()) *= Mat4::rotation_x(0.7854);
    assert!(graph.changed(graph.id()));
    assert!(!graph.changed(xa));
    assert!(!graph.changed(xb));
    assert!(!graph.changed(xaa));

    graph.update_world();
    assert!(!graph.changed(graph.id()));
    assert!(!graph.changed(xa));
    assert!(!graph.changed(xb));
    assert!(!graph.changed(xaa));
}

#[test]
fn changed_upward() {
    let mut graph = Transform::default();
    assert!(!graph.changed_upward(graph.id()));

    let xa = graph.insert(Mat4::from(1.0), graph.id());
    assert!(!graph.changed_upward(graph.id()));
    assert!(graph.changed_upward(xa));

    graph.update_world();
    assert!(!graph.changed_upward(graph.id()));
    assert!(!graph.changed_upward(xa));

    let xb = graph.insert(Mat4::from(1.0), graph.id());
    let xc = graph.insert(Mat4::from(1.0), graph.id());
    let xbb = graph.insert(Mat4::from(1.0), xb);
    assert!(!graph.changed_upward(graph.id()));
    assert!(!graph.changed_upward(xa));
    assert!(graph.changed_upward(xb));
    assert!(graph.changed_upward(xc));
    assert!(graph.changed_upward(xbb));

    graph.update_world();
    assert!(!graph.changed_upward(graph.id()));
    assert!(!graph.changed_upward(xa));
    assert!(!graph.changed_upward(xb));
    assert!(!graph.changed_upward(xc));
    assert!(!graph.changed_upward(xbb));

    *graph.local_mut(xa) *= Mat4::rotation_y(0.7854);
    assert!(!graph.changed_upward(graph.id()));
    assert!(graph.changed_upward(xa));
    assert!(!graph.changed_upward(xb));
    assert!(!graph.changed_upward(xc));
    assert!(!graph.changed_upward(xbb));

    *graph.local_mut(xb) *= Mat4::rotation_y(-0.7854);
    assert!(!graph.changed_upward(graph.id()));
    assert!(graph.changed_upward(xa));
    assert!(graph.changed_upward(xb));
    assert!(!graph.changed_upward(xc));
    assert!(graph.changed_upward(xbb));

    let xbbb = graph.insert(Mat4::from(1.0), xbb);
    assert!(!graph.changed_upward(graph.id()));
    assert!(graph.changed_upward(xa));
    assert!(graph.changed_upward(xb));
    assert!(!graph.changed_upward(xc));
    assert!(graph.changed_upward(xbb));
    assert!(graph.changed_upward(xbbb));

    graph.update_world();
    assert!(!graph.changed_upward(graph.id()));
    assert!(!graph.changed_upward(xa));
    assert!(!graph.changed_upward(xb));
    assert!(!graph.changed_upward(xc));
    assert!(!graph.changed_upward(xbb));
    assert!(!graph.changed_upward(xbbb));

    *graph.local_mut(xb) *= Mat4::rotation_y(0.7854);
    assert!(!graph.changed_upward(graph.id()));
    assert!(!graph.changed_upward(xa));
    assert!(graph.changed_upward(xb));
    assert!(!graph.changed_upward(xc));
    assert!(graph.changed_upward(xbb));
    assert!(graph.changed_upward(xbbb));

    *graph.local_mut(graph.id()) *= Mat4::translation(0.0, 0.0, -50.0);
    assert!(graph.changed_upward(graph.id()));
    assert!(graph.changed_upward(xa));
    assert!(graph.changed_upward(xb));
    assert!(graph.changed_upward(xc));
    assert!(graph.changed_upward(xbb));
    assert!(graph.changed_upward(xbbb));

    graph.update_world();
    assert!(!graph.changed_upward(graph.id()));
    assert!(!graph.changed_upward(xa));
    assert!(!graph.changed_upward(xb));
    assert!(!graph.changed_upward(xc));
    assert!(!graph.changed_upward(xbb));
    assert!(!graph.changed_upward(xbbb));

    *graph.local_mut(graph.id()) *= Mat4::translation(0.0, 0.0, 50.0);
    assert!(graph.changed_upward(graph.id()));
    assert!(graph.changed_upward(xa));
    assert!(graph.changed_upward(xb));
    assert!(graph.changed_upward(xc));
    assert!(graph.changed_upward(xbb));
    assert!(graph.changed_upward(xbbb));
}
