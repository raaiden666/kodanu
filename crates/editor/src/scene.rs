use ecs::{DynamicBundle, Entity, World};

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
    pub fn spawn<B>(&mut self, bundle: B)
    where
        B: DynamicBundle,
    {
        self.world.spawn(bundle);
    }

    pub fn despawn(&mut self, entity: Entity) {
        let _ = self.world.despawn(entity);
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
