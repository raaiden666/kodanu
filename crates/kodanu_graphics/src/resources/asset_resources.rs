use crate::{
    GpuMaterial, MaterialCache, RenderItem,
    mesh::{GpuMesh, MeshCache},
};

use {
    std::sync::Arc,
    wgpu::{BindGroupLayout, Device},
};

#[derive(Default)]
pub(crate) struct AssetResources {
    mesh: MeshCache,
    material: MaterialCache,
}

impl AssetResources {
    pub fn gpu_mesh(&mut self, device: &Device, item: &RenderItem) -> Arc<GpuMesh> {
        self.mesh.get_or_create(device, &item.mesh_handle())
    }

    pub fn gpu_material(
        &mut self,
        device: &Device,
        bind_group_layout: &BindGroupLayout,
        item: &RenderItem,
    ) -> Arc<GpuMaterial> {
        self.material
            .get_or_create(device, bind_group_layout, &item.material_handle())
    }
}
