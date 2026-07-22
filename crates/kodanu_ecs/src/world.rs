use crate::{Component, ComponentStorage, EntityAllocator, SparseSet, entity::Entity};

use {hashbrown::HashMap, std::any::TypeId};

#[derive(Default)]
pub struct World {
    entities: EntityAllocator,
    storages: HashMap<TypeId, Box<dyn ComponentStorage>>,
}

impl World {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entities: EntityAllocator::with_capacity(capacity),
            storages: HashMap::with_capacity(capacity),
        }
    }

    pub fn spawn(&mut self) -> Entity {
        self.entities.create()
    }

    pub fn despawn(&mut self, entity: Entity) -> bool {
        if !self.entities.destroy(entity) {
            return false;
        }

        for storage in self.storages.values_mut() {
            storage.remove_entity(entity.index);
        }

        true
    }

    pub fn insert<T: Component>(&mut self, entity: Entity, component: T) -> Option<T> {
        self.assert_alive(entity);

        self.storage_entry::<T>().insert(entity.index, component)
    }

    pub fn remove<T: Component>(&mut self, entity: Entity) -> Option<T> {
        if !self.entities.is_alive(entity) {
            return None;
        }

        self.storage_entry::<T>().remove(entity.index)
    }

    pub fn get<T: Component>(&self, entity: Entity) -> Option<&T> {
        if !self.entities.is_alive(entity) {
            return None;
        }

        self.storage::<T>()?.get(entity.index)
    }

    pub fn get_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
        if !self.entities.is_alive(entity) {
            return None;
        }

        self.storage_entry::<T>().get_mut(entity.index)
    }

    fn storage<T: Component>(&self) -> Option<&SparseSet<T>> {
        self.storages
            .get(&TypeId::of::<T>())?
            .as_any()
            .downcast_ref::<SparseSet<T>>()
    }

    fn storage_entry<T: Component>(&mut self) -> &mut SparseSet<T> {
        self.storages
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Box::new(SparseSet::<T>::default()))
            .as_any_mut()
            .downcast_mut::<SparseSet<T>>()
            .unwrap()
    }

    pub fn assert_alive(&self, entity: Entity) {
        assert!(self.entities.is_alive(entity), "Entity not found");
    }
}
