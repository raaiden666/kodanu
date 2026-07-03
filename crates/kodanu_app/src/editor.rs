#![allow(dead_code)]

use {
    kodanu_assets::{Material, Mesh},
    kodanu_color::Color,
    kodanu_editor::{Scene, SceneCamera},
    kodanu_graphics::RenderItem,
    kodanu_input::{ActionMap, Input},
    kodanu_math::{Quat, Vec3},
    kodanu_scene::MeshRenderer,
    kodanu_time::Time,
    kodanu_transform::Transform,
};

#[derive(Default)]
pub(crate) struct Editor {
    scene: Scene,
    scene_camera: SceneCamera,
}

impl Editor {
    pub fn update(&mut self, input: &Input, action_map: &ActionMap, time: &Time) {
        self.scene_camera.update(input, action_map, time);
    }

    pub fn collect_render_items(&mut self) -> Vec<RenderItem> {
        let mut items = Vec::with_capacity(12);

        let mut query = self.scene.world().query::<(&Transform, &MeshRenderer)>();

        for (transform, mesh_renderer) in query.iter() {
            let (mesh, material, model) = (
                mesh_renderer.mesh_handle(),
                mesh_renderer.material_handle(),
                transform.matrix(),
            );

            items.push(RenderItem::new(mesh, material, model));
        }

        items
    }

    pub(crate) fn init_test_mesh(&mut self) {
        self.scene_camera
            .transform_mut()
            .set_position(Vec3::new(0.0, 0.0, 5.0));

        self.scene.world_mut().spawn((
            Transform::default(),
            MeshRenderer::new(Mesh::cube_2d(), Material::new(Color::GREEN)),
        ));

        let mut tranform = Transform::default();

        tranform.set_position(Vec3::new(-2.0, 0.0, -1.0));
        tranform.set_rotation(Quat::from_rotation_y(45.0));

        self.scene.world_mut().spawn((
            tranform,
            MeshRenderer::new(Mesh::triangle_2d(), Material::new(Color::BLUE)),
        ));
    }
}

impl Editor {
    #[inline]
    pub fn scene(&self) -> &Scene {
        &self.scene
    }

    #[inline]
    pub fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }

    #[inline]
    pub fn scene_camera(&self) -> &SceneCamera {
        &self.scene_camera
    }

    #[inline]
    pub fn scene_camera_mut(&mut self) -> &mut SceneCamera {
        &mut self.scene_camera
    }
}
