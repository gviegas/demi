// Copyright 2022 Gustavo C. Viegas. All rights reserved.

//! Drawable entity.

use std::sync::Arc;

use crate::mesh::Mesh;
use crate::shape::{Bbox, Sphere};
use crate::skin::Skin;

/// Drawable.
#[derive(Debug)]
pub struct Drawable {
    mesh: Arc<Mesh>,
    shape: Shape,
    // TODO: Skin instancing.
    skin: Option<Arc<Skin>>,
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
    pub fn new(mesh: Arc<Mesh>, shape: Shape) -> Self {
        Self {
            mesh,
            shape,
            skin: None,
        }
    }

    /// Creates a new drawable with a [`Skin`].
    pub fn new_skinned(mesh: Arc<Mesh>, shape: Shape, skin: Arc<Skin>) -> Self {
        Self {
            mesh,
            shape,
            skin: Some(skin),
        }
    }

    /// Returns a reference to the reference-counted [`Mesh`].
    pub fn mesh(&self) -> &Arc<Mesh> {
        &self.mesh
    }

    /// Returns the [`Shape`].
    pub fn shape(&self) -> Shape {
        self.shape
    }

    /// Returns a reference to the reference-counted [`Skin`],
    /// or `None` if the drawable has no skin.
    pub fn skin(&self) -> Option<&Arc<Skin>> {
        self.skin.as_ref()
    }
}
