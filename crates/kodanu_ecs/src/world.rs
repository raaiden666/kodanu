use crate::{
    Component, ComponentStorage, EntityAllocator, Query, QueryIter, QueryMut, SparseSet,
    entity::Entity,
};

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

    #[inline]
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

    #[inline]
    pub fn get<T: Component>(&self, entity: Entity) -> Option<&T> {
        if !self.entities.is_alive(entity) {
            return None;
        }

        self.storage::<T>()?.get(entity.index)
    }

    #[inline]
    pub fn get_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
        if !self.entities.is_alive(entity) {
            return None;
        }

        self.storage_entry::<T>().get_mut(entity.index)
    }

    #[inline]
    pub fn storage<T: Component>(&self) -> Option<&SparseSet<T>> {
        self.storages
            .get(&TypeId::of::<T>())?
            .as_any()
            .downcast_ref::<SparseSet<T>>()
    }

    #[inline]
    pub fn storage_mut<T: Component>(&mut self) -> Option<&mut SparseSet<T>> {
        self.storages
            .get_mut(&TypeId::of::<T>())
            .and_then(|storage| storage.as_any_mut().downcast_mut::<SparseSet<T>>())
    }

    #[inline]
    pub(crate) fn storage_ptr<T: Component>(&self) -> Option<*const SparseSet<T>> {
        let storage = self.storages.get(&TypeId::of::<T>())?;
        let storage = storage.as_any().downcast_ref::<SparseSet<T>>()?;

        Some(storage as *const SparseSet<T>)
    }

    #[inline]
    pub(crate) fn storage_mut_ptr<T: Component>(&mut self) -> Option<*mut SparseSet<T>> {
        let storage = self.storages.get_mut(&TypeId::of::<T>())?;
        let storage = storage.as_any_mut().downcast_mut::<SparseSet<T>>()?;

        Some(storage as *mut SparseSet<T>)
    }

    #[inline]
    pub fn assert_alive(&self, entity: Entity) {
        assert!(self.entities.is_alive(entity), "Entity not found");
    }

    fn storage_entry<T: Component>(&mut self) -> &mut SparseSet<T> {
        self.storages
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Box::new(SparseSet::<T>::default()))
            .as_any_mut()
            .downcast_mut::<SparseSet<T>>()
            .unwrap()
    }
}

impl World {
    #[inline]
    pub fn query<'w, Q>(&'w self) -> Option<QueryIter<'w, Q::Fetch<'w>>>
    where
        Q: Query,
    {
        let fetch = Q::create_fetch(self)?;

        Some(QueryIter::new(fetch))
    }

    #[inline]
    pub fn query_mut<'w, Q>(&'w mut self) -> Option<QueryIter<'w, Q::Fetch<'w>>>
    where
        Q: QueryMut,
    {
        let fetch = Q::create_fetch_mut(self)?;

        Some(QueryIter::new(fetch))
    }
}
