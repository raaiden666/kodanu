use ecs::{DynamicBundle, Entity, World};

#[derive(Default)]
pub struct Scene {
    world: World,
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
}
