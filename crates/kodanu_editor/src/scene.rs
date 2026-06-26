use kodanu_ecs::World;

#[derive(Default)]
pub struct Scene {
    world: World,
}

impl Scene {
    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }
}
