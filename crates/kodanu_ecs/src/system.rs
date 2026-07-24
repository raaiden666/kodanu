use crate::UnsafeWorldCell;

pub trait System {
    fn run(&mut self, world: UnsafeWorldCell);
}
