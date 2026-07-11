#![allow(dead_code)]

use {
    kodanu_editor::{Scene, SceneCamera},
    kodanu_graphics::RenderItem,
    kodanu_input::{ActionMap, Input},
    kodanu_math::Vec3,
    kodanu_scene::MeshRenderer,
    kodanu_time::Time,
    kodanu_transform::Transform,
};

#[derive(Default)]
pub(crate) struct Editor {
    scene: Scene,
    scene_camera: SceneCamera,
    items: Vec<RenderItem>,
}

impl Editor {
    pub fn init(&mut self) {
        self.scene_camera
            .transform_mut()
            .set_position(Vec3::new(0.0, 0.0, 5.0));
    }

    pub fn update(&mut self, input: &Input, action_map: &ActionMap, time: &Time) {
        self.scene_camera.update(input, action_map, time);
    }

    pub fn collect_render_items(&mut self) -> &[RenderItem] {
        self.items.clear();

        for (transform, mesh_renderer) in self
            .scene
            .world()
            .query::<(&Transform, &MeshRenderer)>()
            .iter()
        {
            let (mesh, material, model) = (
                mesh_renderer.mesh_handle(),
                mesh_renderer.material_handle(),
                transform.matrix(),
            );

            self.items.push(RenderItem::new(mesh, material, model));
        }

        &self.items
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
