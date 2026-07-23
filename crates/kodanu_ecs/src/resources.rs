use std::any::{Any, TypeId};

use hashbrown::HashMap;

#[derive(Default)]
pub struct Resources {
    values: HashMap<TypeId, Box<dyn Any>>,
}
