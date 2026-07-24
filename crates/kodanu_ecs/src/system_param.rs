use crate::{UnsafeWorldCell, World};

pub trait SystemParam {
    type State;

    type Item<'w>;

    fn init(world: &World) -> Self::State;

    fn fetch<'w>(state: &'w mut Self::State, world: UnsafeWorldCell<'w>) -> Self::Item<'w>;
}
