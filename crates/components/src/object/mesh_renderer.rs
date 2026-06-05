use graphics::Mesh;

use std::sync::Arc;

pub struct MeshRenderer {
    pub mesh: Arc<Mesh>,
}

impl MeshRenderer {
    pub fn new(mesh: Mesh) -> Self {
        Self {
            mesh: Arc::new(mesh),
        }
    }

    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }
}
