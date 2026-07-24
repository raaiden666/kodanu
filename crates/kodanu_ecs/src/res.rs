use std::ops::Deref;

use crate::{Resource, SystemParam, World};

pub struct Res<'w, T: Resource> {
    value: &'w T,
}

impl<'w, T: Resource> Res<'w, T> {
    #[inline]
    pub fn get(&self) -> &T {
        self.value
    }
}

impl<T: Resource> SystemParam for Res<'_, T> {
    type State = ();

    type Item<'w> = Res<'w, T>;

    fn init(_: &World) -> Self::State {}

    fn fetch<'w>(_: &mut Self::State, world: crate::UnsafeWorldCell<'w>) -> Self::Item<'w> {
        let value = unsafe { world.res::<T>() }.expect("Resource not found!");

        Res { value }
    }
}

impl<'w, T: Resource> Deref for Res<'w, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}
