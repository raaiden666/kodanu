use crate::{SystemParam, SystemParamFunction, UnsafeWorldCell, all_tuples};

#[macro_export]
macro_rules! impl_system_function {
    ($($name:ident : $index:tt),*) => {
        impl<F, $($name),*> SystemParamFunction<fn($($name),*)> for F
        where $($name: SystemParam,)*
        F: for<'w> FnMut(
            $(<$name as SystemParam>::Item<'w>,)*
        ) {
            type Param = ($($name,)*);

            #[inline]
            fn run(&mut self,
                state: &mut <Self::Param as SystemParam>::State,
                world: UnsafeWorldCell
            ) {
                let ($($name,)*) = <Self::Param as SystemParam>::fetch(state, world);

                (self)($($name),*)
            }
        }
    }
}

all_tuples!(impl_system_function);
