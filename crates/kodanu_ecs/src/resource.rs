use std::any::Any;

pub trait Resource: Any + Send + Sync {}

impl<T: Any + Send + Sync> Resource for T {}
