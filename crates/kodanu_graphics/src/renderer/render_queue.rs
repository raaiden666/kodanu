use crate::RenderItem;

use {kodanu_editor::Scene, kodanu_scene::MeshRenderer, kodanu_transform::Transform};

#[derive(Default)]
pub struct RenderQueue {
    render_items: Vec<RenderItem>,
}

impl RenderQueue {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            render_items: Vec::with_capacity(capacity),
        }
    }

    pub fn collect_render_items(&mut self, scene: &Scene) {
        self.render_items.clear();

        for (transform, mesh_renderer) in
            scene.world().query::<(&Transform, &MeshRenderer)>().iter()
        {
            let (mesh, material, model) = (
                mesh_renderer.mesh_handle(),
                mesh_renderer.material_handle(),
                transform.matrix(),
            );

            self.render_items
                .push(RenderItem::new(mesh, material, model));
        }
    }
}

impl RenderQueue {
    #[inline]
    pub fn items(&self) -> &[RenderItem] {
        &self.render_items
    }
}
