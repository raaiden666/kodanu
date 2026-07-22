use std::any::Any;

pub trait ComponentStorage: Any {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn remove_entity(&mut self, entity_index: u32);
}
