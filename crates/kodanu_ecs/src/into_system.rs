use crate::{FunctionSystem, System, SystemParamFunction, World};

pub trait IntoSystem<Marker> {
    type System: System;

    fn into_system(self, world: &World) -> Self::System;
}

impl<F, Marker> IntoSystem<Marker> for F
where
    F: SystemParamFunction<Marker>,
{
    type System = FunctionSystem<F, Marker>;

    fn into_system(self, world: &World) -> Self::System {
        FunctionSystem::new(self, world)
    }
}
