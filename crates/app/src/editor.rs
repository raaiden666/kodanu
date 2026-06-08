use {
    assets::Mesh,
    components::{MeshRenderer, Transform},
    editor::{Scene, SceneCamera},
    graphics::RenderItem,
    input::Input,
    math::Quat,
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

    pub fn init_test_mesh(&mut self) {
        self.scene
            .world_mut()
            .spawn((Transform::default(), MeshRenderer::new(Mesh::triangle_2d())));
    }

    pub fn rotate_all_meshes(&mut self, speed: f32) {
        let mut query = self.scene.world_mut().query::<&mut Transform>();

        for transform in query.iter() {
            transform.rotate(Quat::from_rotation_z(speed));
        }
    }
}
