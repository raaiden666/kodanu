use crate::{Resource, SystemParam, UnsafeWorldCell};

use std::ops::{Deref, DerefMut};

pub struct ResMut<'w, T: Resource> {
    value: &'w mut T,
}

impl<T: Resource> SystemParam for ResMut<'_, T> {
    type State = ();
    type Item<'w> = ResMut<'w, T>;

    fn init(_: &crate::World) -> Self::State {}

    fn fetch<'w>(_: &'w mut Self::State, world: UnsafeWorldCell<'w>) -> Self::Item<'w> {
        let value = unsafe { world.res_mut::<T>() }.expect("Resource not forund!");

        ResMut { value }
    }
}

impl<'w, T: Resource> Deref for ResMut<'w, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<'w, T: Resource> DerefMut for ResMut<'w, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value
    }
}
