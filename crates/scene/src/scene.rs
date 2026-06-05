use ecs::World;

pub struct Scene {
    world: World,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            world: World::new(),
        }
    }
}

impl Scene {
    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }
}
