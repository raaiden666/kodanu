use crate::{Component, Fetch, SparseSet};

use std::marker::PhantomData;

pub struct ReadFetch<'w, T: Component> {
    sparse: &'w [u32],
    dense: *const T,
    entities: &'w [u32],
    marker: PhantomData<&'w T>,
}

impl<'w, T: Component> ReadFetch<'w, T> {
    #[inline]
    pub(crate) fn new(storage: &'w SparseSet<T>) -> Self {
        Self {
            sparse: storage.sparse(),
            dense: storage.dense().as_ptr(),
            entities: storage.indices(),
            marker: PhantomData,
        }
    }
}

impl<'w, T: Component> Fetch<'w> for ReadFetch<'w, T> {
    type Item = &'w T;

    #[inline]
    fn get(&mut self, entity_index: u32) -> Option<Self::Item> {
        let dense_index = *self.sparse.get(entity_index as usize)?;

        if dense_index == u32::MAX {
            return None;
        }

        Some(unsafe { &*self.dense.add(dense_index as usize) })
    }

    #[inline]
    fn entities(&self) -> &[u32] {
        self.entities
    }
}
