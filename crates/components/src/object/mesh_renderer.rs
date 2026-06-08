use graphics::GpuMesh;

use std::sync::Arc;

pub struct MeshRenderer {
    mesh: Arc<GpuMesh>,
}

impl MeshRenderer {
    pub fn new(mesh: GpuMesh) -> Self {
        Self {
            mesh: Arc::new(mesh),
        }
    }

    pub fn mesh(&self) -> &GpuMesh {
        &self.mesh.as_ref()
    }

    pub fn mesh_handle(&self) -> Arc<GpuMesh> {
        Arc::clone(&self.mesh)
    }
}
