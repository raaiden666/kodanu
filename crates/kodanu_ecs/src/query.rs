use crate::{Component, Fetch, ReadFetch, World};

pub trait Query {
    type Item<'w>;
    type Fetch<'w>: Fetch<'w, Item = Self::Item<'w>>;

    fn create_fetch<'w>(world: &'w World) -> Option<Self::Fetch<'w>>;
}

impl<T: Component> Query for &T {
    type Item<'w> = &'w T;
    type Fetch<'w> = ReadFetch<'w, T>;

    #[inline]
    fn create_fetch<'w>(world: &'w World) -> Option<Self::Fetch<'w>> {
        let t = world.storage::<T>()?;

        Some(ReadFetch::new(t))
    }
}

impl<A, B> Query for (&A, &B)
where
    A: Component,
    B: Component,
{
    type Item<'w> = (&'w A, &'w B);
    type Fetch<'w> = (ReadFetch<'w, A>, ReadFetch<'w, B>);

    #[inline]
    fn create_fetch<'w>(world: &'w World) -> Option<Self::Fetch<'w>> {
        let a = world.storage::<A>()?;
        let b = world.storage::<B>()?;

        Some((ReadFetch::new(a), ReadFetch::new(b)))
    }
}
