use crate::{Component, Entity, SparseSet, World, WorldAccess};

use std::{marker::PhantomData, ptr::NonNull};

#[derive(Clone, Copy)]
pub struct UnsafeWorldCell<'w> {
    world: NonNull<World>,
    marker: PhantomData<&'w mut World>,
    access: WorldAccess,
}

impl<'w> UnsafeWorldCell<'w> {
    #[inline]
    pub(crate) fn from_world(world: &'w World) -> Self {
        Self {
            world: NonNull::from(world).cast(),
            marker: PhantomData,
            access: WorldAccess::ReadOnly,
        }
    }

    #[inline]
    pub(crate) fn from_world_mut(world: &'w mut World) -> Self {
        Self {
            world: NonNull::from(world),
            marker: PhantomData,
            access: WorldAccess::ReadWrite,
        }
    }
}

impl<'w> UnsafeWorldCell<'w> {
    /// # SAFETY
    #[inline]
    pub unsafe fn storage<T: Component>(&self) -> Option<&'w SparseSet<T>> {
        unsafe { self.world.as_ref().storage::<T>() }
    }

    /// # SAFETY
    #[inline]
    pub unsafe fn storage_mut<T: Component>(&self) -> Option<&'w mut SparseSet<T>> {
        self.assert_mutable();

        unsafe {
            self.world
                .as_ptr()
                .as_mut()
                .unwrap_unchecked()
                .storage_mut::<T>()
        }
    }

    /// # SAFETY
    #[inline]
    pub unsafe fn storage_ptr<T: Component>(&self) -> Option<*const SparseSet<T>> {
        unsafe { self.world.as_ref().storage_ptr::<T>() }
    }

    /// # SAFETY
    #[inline]
    pub unsafe fn storage_mut_ptr<T: Component>(&self) -> Option<*mut SparseSet<T>> {
        self.assert_mutable();

        unsafe {
            self.world
                .as_ptr()
                .as_mut()
                .unwrap_unchecked()
                .storage_mut_ptr::<T>()
        }
    }

    /// # SAFETY
    #[inline]
    pub unsafe fn res<T: Component>(&self) -> Option<&'w T> {
        unsafe { self.world.as_ref().res::<T>() }
    }

    /// # SAFETY
    #[inline]
    pub unsafe fn res_mut<T: Component>(&self) -> Option<&'w mut T> {
        self.assert_mutable();

        unsafe {
            self.world
                .as_ptr()
                .as_mut()
                .unwrap_unchecked()
                .res_mut::<T>()
        }
    }

    /// # SAFETY
    #[inline]
    pub unsafe fn spawn(&self) -> Entity {
        self.assert_mutable();

        unsafe { self.world.as_ptr().as_mut().unwrap_unchecked().spawn() }
    }

    /// # SAFETY
    #[inline]
    pub unsafe fn despawn(&self, entity: Entity) -> bool {
        self.assert_mutable();

        unsafe {
            self.world
                .as_ptr()
                .as_mut()
                .unwrap_unchecked()
                .despawn(entity)
        }
    }

    /// # SAFETY
    #[inline]
    pub unsafe fn get<T: Component>(&self, entity: Entity) -> Option<&'w T> {
        unsafe { self.world.as_ref().get(entity) }
    }

    /// # SAFETY
    #[inline]
    pub unsafe fn get_mut<T: Component>(&self, entity: Entity) -> Option<&'w mut T> {
        self.assert_mutable();

        unsafe {
            self.world
                .as_ptr()
                .as_mut()
                .unwrap_unchecked()
                .get_mut(entity)
        }
    }
    #[inline]
    fn assert_mutable(&self) {
        debug_assert!(
            matches!(self.access, WorldAccess::ReadWrite),
            "Attempt to mutably a read-only World"
        );
    }
}
