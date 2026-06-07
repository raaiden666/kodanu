use crate::resources::Mesh;

use std::sync::Arc;

use math::Mat4;

pub struct RenderItem {
    mesh: Arc<Mesh>,
    model: Mat4,
}

impl RenderItem {
    pub fn new(mesh: Arc<Mesh>, model: Mat4) -> Self {
        Self { mesh, model }
    }
}

impl RenderItem {
    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }

    pub fn model(&self) -> Mat4 {
        self.model
    }
}
