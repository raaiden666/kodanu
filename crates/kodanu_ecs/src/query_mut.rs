use crate::{Component, Fetch, ReadFetch, World, WriteFetch};

use std::any::TypeId;

pub trait QueryMut {
    type Item<'w>;

    type Fetch<'w>: Fetch<'w, Item = Self::Item<'w>>;

    fn create_fetch_mut<'w>(world: &'w mut World) -> Option<Self::Fetch<'w>>;
}

impl<T: Component> QueryMut for &mut T {
    type Item<'w> = &'w mut T;

    type Fetch<'w> = WriteFetch<'w, T>;

    #[inline]
    fn create_fetch_mut<'w>(world: &'w mut World) -> Option<Self::Fetch<'w>> {
        let t = world.storage_mut::<T>()?;

        Some(WriteFetch::new(t))
    }
}

impl<A, B> QueryMut for (&mut A, &B)
where
    A: Component,
    B: Component,
{
    type Item<'w> = (&'w mut A, &'w B);
    type Fetch<'w> = (WriteFetch<'w, A>, ReadFetch<'w, B>);

    #[inline]
    fn create_fetch_mut<'w>(world: &'w mut World) -> Option<Self::Fetch<'w>> {
        assert_ne::<A, B>();

        let world_ptr = world as *mut World;

        let a = unsafe { (*world_ptr).storage_mut_ptr::<A>()? };
        let b = unsafe { (*world_ptr).storage_ptr::<B>()? };

        unsafe { Some((WriteFetch::from_ptr(a), ReadFetch::from_ptr(b))) }
    }
}

impl<A, B> QueryMut for (&A, &mut B)
where
    A: Component,
    B: Component,
{
    type Item<'w> = (&'w A, &'w mut B);
    type Fetch<'w> = (ReadFetch<'w, A>, WriteFetch<'w, B>);

    #[inline]
    fn create_fetch_mut<'w>(world: &'w mut World) -> Option<Self::Fetch<'w>> {
        assert_ne::<A, B>();

        let world_ptr = world as *mut World;

        let a = unsafe { (*world_ptr).storage_ptr::<A>()? };
        let b = unsafe { (*world_ptr).storage_mut_ptr::<B>()? };

        unsafe { Some((ReadFetch::from_ptr(a), WriteFetch::from_ptr(b))) }
    }
}

#[inline]
pub fn assert_ne<A, B>()
where
    A: Component,
    B: Component,
{
    assert_ne!(
        TypeId::of::<A>(),
        TypeId::of::<B>(),
        "Query aliases the same component!"
    );
}
