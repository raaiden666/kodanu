use crate::{Component, Fetch, ReadFetch, UnsafeWorldCell, WriteFetch};

pub trait Query {
    type Item<'w>;
    type Fetch<'w>: Fetch<'w, Item = Self::Item<'w>>;

    fn create_fetch<'w>(world: UnsafeWorldCell<'w>) -> Option<Self::Fetch<'w>>;
}

impl<T: Component> Query for &T {
    type Item<'w> = &'w T;
    type Fetch<'w> = ReadFetch<'w, T>;

    #[inline]
    fn create_fetch<'w>(world: UnsafeWorldCell<'w>) -> Option<Self::Fetch<'w>> {
        let t = unsafe { world.storage::<T>()? };

        Some(ReadFetch::new(t))
    }
}

impl<T: Component> Query for &mut T {
    type Item<'w> = &'w mut T;
    type Fetch<'w> = WriteFetch<'w, T>;

    #[inline]
    fn create_fetch<'w>(world: UnsafeWorldCell<'w>) -> Option<Self::Fetch<'w>> {
        let ptr = unsafe { world.storage_mut_ptr::<T>()? };

        unsafe { Some(WriteFetch::from_ptr(ptr)) }
    }
}

impl<A, B> Query for (A, B)
where
    A: Query,
    B: Query,
{
    type Item<'w> = (A::Item<'w>, B::Item<'w>);
    type Fetch<'w> = (A::Fetch<'w>, B::Fetch<'w>);

    #[inline]
    fn create_fetch<'w>(world: UnsafeWorldCell<'w>) -> Option<Self::Fetch<'w>> {
        Some((A::create_fetch(world)?, B::create_fetch(world)?))
    }
}
