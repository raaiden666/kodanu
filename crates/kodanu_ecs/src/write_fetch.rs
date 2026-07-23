use crate::{Component, Fetch, SparseSet};

use std::marker::PhantomData;

pub struct WriteFetch<'w, T: Component> {
    sparse: &'w [u32],
    dense: *mut T,
    entities: &'w [u32],
    marker: PhantomData<&'w mut T>,
}

impl<'w, T: Component> WriteFetch<'w, T> {
    #[inline]
    pub fn new(storage: &'w mut SparseSet<T>) -> Self {
        let storage_ptr = storage as *mut SparseSet<T>;

        unsafe {
            Self {
                sparse: &(*storage_ptr).sparse,
                dense: (*storage_ptr).dense.as_mut_ptr(),
                entities: (*storage).indices(),
                marker: PhantomData,
            }
        }
    }

    #[inline]
    pub(crate) unsafe fn from_ptr(storage: *mut SparseSet<T>) -> Self {
        Self {
            sparse: unsafe { (*storage).sparse() },
            dense: unsafe { (*storage).dense_mut().as_mut_ptr() },
            entities: unsafe { (*storage).indices() },
            marker: PhantomData,
        }
    }
}

impl<'w, T: Component> Fetch<'w> for WriteFetch<'w, T> {
    type Item = &'w mut T;

    #[inline]
    fn get(&mut self, entity_index: u32) -> Option<Self::Item> {
        let dense_index = *self.sparse.get(entity_index as usize)?;

        if dense_index == u32::MAX {
            return None;
        }

        Some(unsafe { &mut *self.dense.add(dense_index as usize) })
    }

    #[inline]
    fn entities(&self) -> &[u32] {
        self.entities
    }
}
