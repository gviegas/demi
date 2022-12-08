// Copyright 2022 Gustavo C. Viegas. All rights reserved.

//! Graph of transformation matrices.

use crate::linear::Mat4;

#[cfg(test)]
mod tests;

/// Identifier of a transform.
#[derive(Debug)]
pub struct XformId(usize);

/// Node in a transform graph.
#[derive(Debug)]
struct XformNode {
    prev: Option<usize>,
    next: Option<usize>,
    sub: Option<usize>,
    data: usize,
}

/// Data of a transform.
#[derive(Debug)]
struct XformData {
    // TODO: Consider storing the local transform
    // as TRS properties instead.
    local: Mat4<f32>,
    world: Mat4<f32>,
    changed: bool,
    node: usize,
}

/// Transform graph.
#[derive(Debug)]
pub struct Transform {
    nodes: Vec<Option<XformNode>>,
    node_idx: usize,
    none_cnt: usize,
    data: Vec<XformData>,
}

impl Transform {
    /// Creates a new root transform.
    pub fn new(xform: &Mat4<f32>) -> Self {
        Self {
            nodes: vec![Some(XformNode {
                prev: None,
                next: None,
                sub: None,
                data: 0,
            })],
            node_idx: 0,
            none_cnt: 0,
            data: vec![XformData {
                local: xform.clone(),
                world: xform.clone(),
                changed: false,
                node: 0,
            }],
        }
    }

    /// Returns the root transform's identifier.
    pub fn id(&self) -> XformId {
        XformId(0)
    }

    /// Returns the length of the transform graph.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Inserts a new transform.
    ///
    /// NOTE: The `XformId` returned by this method must not be used
    /// with `Transform`s other than the one that produced it.
    pub fn insert(&mut self, prev: &XformId, xform: &Mat4<f32>) -> XformId {
        let new_idx = if self.none_cnt > 0 {
            // There is a vacant node that we can use.
            let n = self.nodes.len();
            let mut i = self.node_idx;
            while self.nodes[i].is_some() {
                i = (i + 1) % n;
            }
            self.node_idx = n / 2;
            self.none_cnt -= 1;
            i
        } else {
            // No vacant nodes, so push a new one.
            let n = self.nodes.len();
            self.nodes.push(None);
            n
        };
        let prev_node = self.nodes[prev.0].as_mut().unwrap();
        let next_idx = prev_node.sub;
        // Insert the new transform as the first child.
        // The current first child, if any, becomes the next sibling.
        prev_node.sub = Some(new_idx);
        if let Some(x) = next_idx {
            self.nodes[x].as_mut().unwrap().prev = Some(new_idx);
        }
        self.nodes[new_idx] = Some(XformNode {
            prev: Some(prev.0),
            next: next_idx,
            sub: None,
            data: self.data.len(),
        });
        self.data.push(XformData {
            local: xform.clone(),
            world: Default::default(),
            changed: true,
            node: new_idx,
        });
        XformId(new_idx)
    }

    /// Removes a given transform.
    ///
    /// Trying to remove the root transform is an error and will cause a panic.
    ///
    /// NOTE: Removing a non-leaf transform does not remove any of its
    /// descendants - they must be explicitly `remove`d.
    pub fn remove(&mut self, id: XformId) {
        assert_ne!(id.0, self.id().0, "cannot remove root transform");
        let node = self.nodes[id.0].take().unwrap();
        self.node_idx = id.0;
        self.none_cnt += 1;
        if let Some(x) = node.prev {
            let prev_sub = self.nodes[x].as_ref().unwrap().sub;
            match prev_sub {
                // `node.prev` is the parent.
                // The next sibling, if any, becomes the first child.
                Some(y) if y == id.0 => self.nodes[x].as_mut().unwrap().sub = node.next,
                // `node.prev` is a sibling.
                // The next sibling, if any, becomes the previous' next sibling.
                _ => self.nodes[x].as_mut().unwrap().next = node.next,
            }
        }
        if let Some(x) = node.next {
            self.nodes[x].as_mut().unwrap().prev = node.prev;
        }
        if let Some(x) = node.sub {
            // NOTE: Orphaned sub-graph.
            self.nodes[x].as_mut().unwrap().prev = None;
        }
        // Unlike nodes, data can be removed from any position, and
        // we just need to update a node's `data` index in the event
        // of a swap-removal.
        let swap = self.data.last().unwrap().node;
        if swap != id.0 {
            self.nodes[swap].as_mut().unwrap().data = node.data;
            self.data.swap_remove(node.data);
        } else {
            self.data.pop();
        }
    }

    /// Returns a reference to a given local transform.
    pub fn local(&self, id: &XformId) -> &Mat4<f32> {
        let data_idx = self.nodes[id.0].as_ref().unwrap().data;
        &self.data[data_idx].local
    }

    /// Returns a mutable reference to a given local transform.
    ///
    /// NOTE: Calling this method will mark `id` as being stale and thus its
    /// world transform (and those of its descendants) will be recomputed
    /// when the graph is updated.
    pub fn local_mut(&mut self, id: &XformId) -> &mut Mat4<f32> {
        let data_idx = self.nodes[id.0].as_ref().unwrap().data;

        // NOTE: Code such as the following can potentially invalidate
        // a whole sub-graph needlessly:
        //
        //  let m = graph.local_mut(&xid);
        //  if <something> { <mutate *m> } else { <don't mutate *m> }
        //
        // It may be better to define a `set_local` method rather than
        // giving mutable access to the transform data, although it
        // would not be possible to patch the local transform in-place.
        self.data[data_idx].changed = true;

        &mut self.data[data_idx].local
    }

    /// Returns a reference to a given world transform.
    pub fn world(&self, id: &XformId) -> &Mat4<f32> {
        let data_idx = self.nodes[id.0].as_ref().unwrap().data;
        &self.data[data_idx].world
    }
}
