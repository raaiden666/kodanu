use std::marker::PhantomData;

use crate::{System, SystemParam, SystemParamFunction, World};

pub struct FunctionSystem<F, Marker>
where
    F: SystemParamFunction<Marker>,
{
    function: F,
    state: <F::Param as SystemParam>::State,
    marker: PhantomData<fn() -> Marker>,
}

impl<F, Marker> FunctionSystem<F, Marker>
where
    F: SystemParamFunction<Marker>,
{
    #[inline]
    pub fn new(function: F, world: &World) -> Self {
        Self {
            function,
            state: <F::Param as SystemParam>::init(world),
            marker: PhantomData,
        }
    }
}

impl<F, Marker> System for FunctionSystem<F, Marker>
where
    F: SystemParamFunction<Marker>,
{
    #[inline]
    fn run(&mut self, world: crate::UnsafeWorldCell) {
        self.function.run(&mut self.state, world);
    }
}
