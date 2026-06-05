use crate::resources::Mesh;

use math::Mat4;

pub struct RenderItem<'a> {
    mesh: &'a Mesh,
    transform: Mat4,
}

impl<'a> RenderItem<'a> {
    pub fn new(mesh: &'a Mesh, transform: Mat4) -> Self {
        Self { mesh, transform }
    }
}

impl<'a> RenderItem<'a> {
    pub fn mesh(&self) -> &Mesh {
        self.mesh
    }

    pub fn transform(&self) -> Mat4 {
        self.transform
    }
}
