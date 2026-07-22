use {
    kodanu_camera::Camera, kodanu_ecs::HecsWorld, kodanu_math::Mat4, kodanu_transform::Transform,
};

#[derive(Default)]
pub struct Scene {
    world: HecsWorld,
}

impl Scene {
    pub fn view_projection(&self) -> Option<Mat4> {
        let mut query = self.world.query::<(&Transform, &Camera)>();

        query
            .iter()
            .next()
            .map(|(transform, camera)| camera.view_projection(transform))
    }

    pub fn set_viewport_size(&mut self, width: u32, height: u32) {
        let mut query = self.world.query::<&mut Camera>();

        if let Some(camera) = query.iter().next() {
            camera.set_viewport_size(width, height);
        }
    }

    #[inline]
    pub fn world(&self) -> &HecsWorld {
        &self.world
    }

    #[inline]
    pub fn world_mut(&mut self) -> &mut HecsWorld {
        &mut self.world
    }
}
