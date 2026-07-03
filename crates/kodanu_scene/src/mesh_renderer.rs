use {
    kodanu_assets::{Material, Mesh},
    std::sync::Arc,
};

pub struct MeshRenderer {
    mesh: Arc<Mesh>,
    material: Arc<Material>,
}

impl MeshRenderer {
    pub fn new(mesh: Mesh, material: Material) -> Self {
        Self {
            mesh: Arc::new(mesh),
            material: Arc::new(material),
        }
    }
}

impl MeshRenderer {
    #[inline]
    pub fn mesh(&self) -> &Mesh {
        self.mesh.as_ref()
    }

    #[inline]
    pub fn mesh_handle(&self) -> Arc<Mesh> {
        Arc::clone(&self.mesh)
    }

    #[inline]
    pub fn material(&self) -> &Material {
        self.material.as_ref()
    }

    #[inline]
    pub fn material_handle(&self) -> Arc<Material> {
        Arc::clone(&self.material)
    }
}
