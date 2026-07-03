use kodanu_ecs::World;

#[derive(Default)]
pub struct Scene {
    world: World,
}

impl Scene {
    #[inline]
    pub fn world(&self) -> &World {
        &self.world
    }

    #[inline]
    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }
}
