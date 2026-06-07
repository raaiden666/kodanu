use graphics::Mesh;

use std::sync::Arc;

pub struct MeshRenderer {
    mesh: Arc<Mesh>,
}

impl MeshRenderer {
    pub fn new(mesh: Mesh) -> Self {
        Self {
            mesh: Arc::new(mesh),
        }
    }

    pub fn mesh(&self) -> &Mesh {
        &self.mesh.as_ref()
    }

    pub fn mesh_handle(&self) -> Arc<Mesh> {
        Arc::clone(&self.mesh)
    }
}
