use {
    assets::Mesh,
    components::{MeshRenderer, Transform},
    editor::{Scene, SceneCamera},
    graphics::RenderItem,
    input::Input,
    math::Vec3,
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
        self.scene_camera
            .transform_mut()
            .set_position(Vec3::new(0.0, 0.0, 50.0));

        self.scene
            .world_mut()
            .spawn((Transform::default(), MeshRenderer::new(Mesh::triangle_2d())));
    }
}

impl Editor {
    pub fn scene(&self) -> &Scene {
        &self.scene
    }

    pub fn scene_camera(&self) -> &SceneCamera {
        &self.scene_camera
    }

    pub fn scene_camera_mut(&mut self) -> &mut SceneCamera {
        &mut self.scene_camera
    }
}
