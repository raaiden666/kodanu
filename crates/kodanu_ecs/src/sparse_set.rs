use std::{any::Any, mem::replace};

use crate::ComponentStorage;

#[derive(Default)]
pub struct SparseSet<T> {
    sparse: Vec<u32>,
    dense_indicies: Vec<u32>,
    dense_componenets: Vec<T>,
}

impl<T> SparseSet<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            sparse: Vec::with_capacity(capacity),
            dense_indicies: Vec::with_capacity(capacity),
            dense_componenets: Vec::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.dense_componenets.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.dense_componenets.is_empty()
    }

    #[inline]
    pub fn contains(&self, entity_index: u32) -> bool {
        self.dense_index(entity_index).is_some()
    }

    #[inline]
    pub fn get(&self, entity_index: u32) -> Option<&T> {
        let dense_index = self.dense_index(entity_index)?;
        Some(&self.dense_componenets[dense_index])
    }

    #[inline]
    pub fn get_mut(&mut self, entity_index: u32) -> Option<&mut T> {
        let dense_index = self.dense_index(entity_index)?;
        Some(&mut self.dense_componenets[dense_index])
    }

    pub fn insert(&mut self, entity_index: u32, component: T) -> Option<T> {
        self.ensure_capacity(entity_index);

        if let Some(dense_index) = self.dense_index(entity_index) {
            return Some(replace(&mut self.dense_componenets[dense_index], component));
        }

        let dense_index = self.dense_componenets.len() as u32;

        self.sparse[entity_index as usize] = dense_index;
        self.dense_indicies.push(entity_index);
        self.dense_componenets.push(component);

        None
    }

    pub fn remove(&mut self, entity_index: u32) -> Option<T> {
        let dense_index = self.dense_index(entity_index)?;
        let last = self.dense_componenets.len() - 1;

        self.sparse[entity_index as usize] = u32::MAX;

        self.dense_indicies.swap(dense_index, last);
        self.dense_componenets.swap(dense_index, last);

        let component = self.dense_componenets.pop().unwrap();

        self.dense_indicies.pop();

        if dense_index != last {
            let moved_entity = self.dense_indicies[dense_index];

            self.sparse[moved_entity as usize] = dense_index as u32;
        }

        Some(component)
    }

    #[inline]
    pub fn entity_index_at(&self, dense_index: usize) -> u32 {
        self.dense_indicies[dense_index]
    }

    #[inline]
    pub fn component_at(&self, dense_index: usize) -> &T {
        &self.dense_componenets[dense_index]
    }

    pub fn clear(&mut self) {
        self.sparse.fill(u32::MAX);
        self.dense_indicies.clear();
        self.dense_componenets.clear();
    }
}

impl<T> SparseSet<T> {
    #[inline]
    fn dense_index(&self, entity_index: u32) -> Option<usize> {
        let dense = *self.sparse.get(entity_index as usize)?;

        if dense == u32::MAX {
            return None;
        }

        Some(dense as usize)
    }

    #[inline]
    fn ensure_capacity(&mut self, entity_index: u32) {
        let required = entity_index as usize + 1;

        if self.sparse.len() < required {
            self.sparse.resize(required, u32::MAX);
        }
    }
}

impl<T: ComponentStorage> ComponentStorage for SparseSet<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn remove_entity(&mut self, entity_index: u32) {
        let _ = self.remove(entity_index);
    }
}
