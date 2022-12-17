// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use std::rc::Rc;

use crate::mesh::Mesh;
use crate::shape::{Bbox, Sphere};
use crate::skin::Skin;
use crate::transform::XformId;

/// Drawable.
#[derive(Debug)]
pub struct Drawable {
    xform: Option<XformId>,
    shape: Shape,
    // TODO: These resources will likely need to use `Arc`.
    mesh: Rc<Mesh>,
    skin: Option<Rc<Skin>>,
    // TODO...
}

/// Shape of a `Drawable`.
#[derive(Copy, Clone, Debug)]
pub enum Shape {
    Bbox(Bbox),
    Sphere(Sphere),
    None,
}

impl Drawable {
    /// Creates a new drawable.
    #[allow(unused_variables)] // TODO
    pub fn new(mesh: Rc<Mesh>, shape: Shape) -> Self {
        todo!();
    }

    /// Creates a new drawable with a `Skin`.
    #[allow(unused_variables)] // TODO
    pub fn new_skinned(mesh: Rc<Mesh>, shape: Shape, skin: Rc<Skin>) -> Self {
        todo!();
    }

    // TODO: Setters.

    /// Returns a reference to the `XformId` or `None` if the drawable
    /// has no transform.
    pub fn xform_id(&self) -> Option<&XformId> {
        self.xform.as_ref()
    }

    /// Returns the `Shape`.
    pub fn shape(&self) -> Shape {
        self.shape
    }

    /// Returns a reference to the reference-counted `Mesh`.
    pub fn mesh(&self) -> &Rc<Mesh> {
        &self.mesh
    }

    /// Returns a reference to the reference-counted `Skin` or `None`
    /// if the drawable has no skin.
    pub fn skin(&self) -> Option<&Rc<Skin>> {
        self.skin.as_ref()
    }
}
