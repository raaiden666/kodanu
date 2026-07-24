use std::any::TypeId;

pub trait SystemParamAccess {
    fn read_types(types: &mut Vec<TypeId>);

    fn write_types(types: &mut Vec<TypeId>);
}
