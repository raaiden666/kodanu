#![allow(dead_code)]

use {
    kodanu_editor::{Scene, SceneCamera},
    kodanu_input::{ActionMap, Input},
    kodanu_math::Vec3,
    kodanu_time::Time,
};

#[derive(Default)]
pub(crate) struct Editor {
    scene: Scene,
    scene_camera: SceneCamera,
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
