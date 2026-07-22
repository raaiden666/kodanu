use std::any::Any;

pub trait Component: Any + Send + Sync {}

impl<T: Any + Send + Sync> Component for T {}
