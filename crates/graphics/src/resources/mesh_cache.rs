use crate::GpuMesh;

use {
    assets::Mesh,
    std::{collections::HashMap, sync::Arc},
    wgpu::Device,
};

pub struct MeshCache {
    meshes: HashMap<usize, Arc<GpuMesh>>,
}

impl MeshCache {
    pub fn new() -> Self {
        Self {
            meshes: HashMap::with_capacity(128),
        }
    }
}

impl MeshCache {
    pub fn get_or_create(&mut self, device: &Device, mesh: &Arc<Mesh>) -> Arc<GpuMesh> {
        let key = Arc::as_ptr(mesh) as usize;

        if let Some(gpu_mesh) = self.meshes.get(&key) {
            return Arc::clone(gpu_mesh);
        }

        let gpu_mesh = Arc::new(GpuMesh::from_mesh(device, mesh));

        self.meshes.insert(key, Arc::clone(&gpu_mesh));

        gpu_mesh
    }
}
