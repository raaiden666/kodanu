use {kodanu_camera::Camera, kodanu_ecs::World, kodanu_math::Mat4, kodanu_transform::Transform};

#[derive(Default)]
pub struct Scene {
    world: World,
}

impl Scene {
    pub fn view_projection(&self) -> Option<Mat4> {
        let mut query = self.world.query::<(&Transform, &Camera)>()?;

        let (transform, camera) = query.next()?;

        Some(camera.view_projection(transform))
    }

    pub fn set_viewport_size(&mut self, width: u32, height: u32) {
        let Some(mut query) = self.world.query_mut::<&mut Camera>() else {
            return;
        };

        if let Some(camera) = query.next() {
            camera.set_viewport_size(width, height);
        }
    }

    #[inline]
    pub fn world(&self) -> &World {
        &self.world
    }

    #[inline]
    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }
}
