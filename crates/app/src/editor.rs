use {editor::SceneCamera, input::Input, time::Time};

pub struct Editor {
    scene_camera: SceneCamera,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            scene_camera: SceneCamera::default(),
        }
    }
}

impl Editor {
    pub fn scene_camera(&self) -> &SceneCamera {
        &self.scene_camera
    }

    pub fn update(&mut self, input: &Input, time: &Time) {
        self.scene_camera.update(input, time);
    }
}
