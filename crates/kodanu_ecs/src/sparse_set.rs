use crate::{Component, ComponentStorage};

use std::{any::Any, mem::replace};

pub struct SparseSet<T> {
    pub(crate) sparse: Vec<u32>,
    pub(crate) indicies: Vec<u32>,
    pub(crate) dense: Vec<T>,
}

impl<T> Default for SparseSet<T> {
    fn default() -> Self {
        Self {
            sparse: Vec::new(),
            indicies: Vec::new(),
            dense: Vec::new(),
        }
    }
}

impl<T> SparseSet<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            sparse: Vec::with_capacity(capacity),
            indicies: Vec::with_capacity(capacity),
            dense: Vec::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.dense.len()
    }

    #[inline]
    pub fn indices(&self) -> &[u32] {
        &self.indicies
    }

    #[inline]
    pub fn dense(&self) -> &[T] {
        &self.dense
    }

    #[inline]
    pub fn dense_mut(&mut self) -> &mut [T] {
        &mut self.dense
    }

    #[inline]
    pub fn sparse(&self) -> &[u32] {
        &self.sparse
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.dense.is_empty()
    }

    #[inline]
    pub fn contains(&self, entity_index: u32) -> bool {
        self.dense_index(entity_index).is_some()
    }

    #[inline]
    pub fn get(&self, entity_index: u32) -> Option<&T> {
        let dense_index = self.dense_index(entity_index)?;
        Some(&self.dense[dense_index])
    }

    #[inline]
    pub fn get_mut(&mut self, entity_index: u32) -> Option<&mut T> {
        let dense_index = self.dense_index(entity_index)?;
        Some(&mut self.dense[dense_index])
    }

    pub fn insert(&mut self, entity_index: u32, component: T) -> Option<T> {
        self.ensure_capacity(entity_index);

        if let Some(dense_index) = self.dense_index(entity_index) {
            return Some(replace(&mut self.dense[dense_index], component));
        }

        let dense_index = self.dense.len() as u32;

        self.sparse[entity_index as usize] = dense_index;
        self.indicies.push(entity_index);
        self.dense.push(component);

        None
    }

    pub fn remove(&mut self, entity_index: u32) -> Option<T> {
        let dense_index = self.dense_index(entity_index)?;
        self.sparse[entity_index as usize] = u32::MAX;

        let component = self.dense.swap_remove(dense_index);
        self.indicies.swap_remove(dense_index);

        if dense_index < self.dense.len() {
            let moved_entity = self.indicies[dense_index];
            self.sparse[moved_entity as usize] = dense_index as u32;
        }

        Some(component)
    }

    pub fn clear(&mut self) {
        self.sparse.fill(u32::MAX);
        self.indicies.clear();
        self.dense.clear();
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

impl<T: Component> ComponentStorage for SparseSet<T> {
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
