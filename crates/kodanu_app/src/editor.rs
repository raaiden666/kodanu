#![allow(dead_code)]

use kodanu_editor::Scene;

#[derive(Default)]
pub(crate) struct Editor {
    scene: Scene,
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
}
