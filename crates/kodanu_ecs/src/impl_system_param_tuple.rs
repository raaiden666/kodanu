use crate::{SystemParam, UnsafeWorldCell, World, all_tuples};

#[macro_export]
macro_rules! impl_system_param_tuple {
    ($($name:ident : $index:tt),+) => {
        impl<$($name),+> SystemParam for ($($name,)+)
        where
            $($name: SystemParam,)+
        {
            type State = ($($name::State,)+);
            type Item<'w> = ($($name::Item<'w>,)+);

            fn init(world: &World) -> Self::State {
                ($($name::init(world),)+)
            }

            fn fetch<'w>(state: &'w mut Self::State, world: UnsafeWorldCell<'w>) -> Self::Item<'w> {
                ($($name::fetch(&mut state.$index, world),)+)
            }
        }
    };
}

all_tuples!(impl_system_param_tuple);
