use crate::{SystemParam, UnsafeWorldCell};

pub trait SystemParamFunction<Marker> {
    type Param: SystemParam;

    fn run(&mut self, state: &mut <Self::Param as SystemParam>::State, world: UnsafeWorldCell);
}
