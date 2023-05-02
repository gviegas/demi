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
#[derive(Copy, Clone, PartialEq, Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linear::{Mat4, Vec3};
    use crate::mesh::{self, DataType, Semantic, Topology};
    use crate::skin;
    use std::io;

    impl Drawable {
        fn check(&self, mesh: &Mesh, shape: &Shape, skin: Option<&Skin>) {
            assert_eq!(Arc::as_ptr(&self.mesh), mesh);
            assert_eq!(self.shape, *shape);
            self.skin.as_ref().map_or_else(
                || assert!(skin.is_none()),
                |x| assert_eq!(Arc::as_ptr(x), skin.unwrap()),
            );
        }
    }

    fn make_mesh() -> Mesh {
        crate::init();
        mesh::Builder::new()
            .set_vertex_count(3)
            .set_semantic(io::repeat(1), Semantic::Position, DataType::F32x3, None)
            .unwrap()
            .push_primitive(Topology::Triangle)
            .unwrap()
            .create()
            .unwrap()
    }

    fn make_skin() -> Skin {
        skin::Builder::new()
            .push_joints(&["Joint"], &[Mat4::from(1.0)], &[None], &[None])
            .unwrap()
            .create()
            .unwrap()
    }

    #[test]
    fn no_skin() {
        let mesh = Arc::new(make_mesh());
        let bbox = Shape::Bbox(Bbox::new(Vec3::from(0.0), Vec3::from(1.0)));
        let sphere = Shape::Sphere(Sphere::new(Vec3::from(0.0), 1.0));

        let d1 = Drawable::new(Arc::clone(&mesh), bbox);
        let d2 = Drawable::new(Arc::clone(&mesh), sphere);
        let d3 = Drawable::new(Arc::clone(&mesh), Shape::None);

        d1.check(&mesh, &bbox, None);
        d2.check(&mesh, &sphere, None);
        d3.check(&mesh, &Shape::None, None);
    }

    #[test]
    fn skinned() {
        let mesh = Arc::new(make_mesh());
        let skin = Arc::new(make_skin());
        let bbox = Shape::Bbox(Bbox::new(Vec3::from(0.0), Vec3::from(1.0)));
        let sphere = Shape::Sphere(Sphere::new(Vec3::from(0.0), 1.0));

        let d1 = Drawable::new_skinned(Arc::clone(&mesh), bbox, Arc::clone(&skin));
        let d2 = Drawable::new_skinned(Arc::clone(&mesh), sphere, Arc::clone(&skin));
        let d3 = Drawable::new_skinned(Arc::clone(&mesh), Shape::None, Arc::clone(&skin));

        d1.check(&mesh, &bbox, Some(&skin));
        d2.check(&mesh, &sphere, Some(&skin));
        d3.check(&mesh, &Shape::None, Some(&skin));
    }
}
