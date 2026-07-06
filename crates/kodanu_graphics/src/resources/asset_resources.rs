use crate::{MaterialCache, mesh::MeshCache};

#[derive(Default)]
pub(crate) struct AssetResources {
    mesh: MeshCache,
    material: MaterialCache,
}

impl AssetResources {
    #[inline]
    pub fn mesh_cache(&mut self) -> &mut MeshCache {
        &mut self.mesh
    }

    #[inline]
    pub fn material_cache(&mut self) -> &mut MaterialCache {
        &mut self.material
    }
}
