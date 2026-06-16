use crate::GpuMaterial;

use {
    assets::Material,
    std::{collections::HashMap, sync::Arc},
    wgpu::BindGroupLayout,
    wgpu::Device,
};

pub(crate) struct MaterialCache {
    materials: HashMap<usize, Arc<GpuMaterial>>,
}

impl MaterialCache {
    pub fn new() -> Self {
        Self {
            materials: HashMap::with_capacity(128),
        }
    }
}

impl MaterialCache {
    pub fn get_or_create(
        &mut self,
        device: &Device,
        bind_group_layout: &BindGroupLayout,
        material: &Arc<Material>,
    ) -> Arc<GpuMaterial> {
        let key = Arc::as_ptr(material) as usize;

        if let Some(gpu_material) = self.materials.get(&key) {
            return Arc::clone(gpu_material);
        }

        let gpu_material = Arc::new(GpuMaterial::new(device, bind_group_layout, material));

        self.materials.insert(key, Arc::clone(&gpu_material));

        gpu_material
    }
}
