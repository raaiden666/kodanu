use {
    components::{MeshRenderer, Transform},
    editor::SceneCamera,
    graphics::RenderItem,
    input::Input,
    scene::Scene,
    time::Time,
};

pub struct Editor {
    scene: Scene,
    scene_camera: SceneCamera,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            scene: Scene::default(),
            scene_camera: SceneCamera::default(),
        }
    }
}

impl Editor {
    pub fn scene(&self) -> &Scene {
        &self.scene
    }

    pub fn scene_camera(&self) -> &SceneCamera {
        &self.scene_camera
    }

    pub fn update(&mut self, input: &Input, time: &Time) {
        self.scene_camera.update(input, time);
    }

    pub fn collect_render_items(&self) -> Vec<RenderItem> {
        let mut items = Vec::new();

        let mut query = self.scene.world().query::<(&Transform, &MeshRenderer)>();

        for (transform, mesh_renderer) in query.iter() {
            items.push(RenderItem::new(
                mesh_renderer.mesh_handle(),
                transform.matrix(),
            ));
        }

        items
    }
}
