use crate::Component;

use std::any::TypeId;

pub trait QueryAccess {
    fn read_types(types: &mut Vec<TypeId>);

    fn write_types(types: &mut Vec<TypeId>);
}

impl<T: Component> QueryAccess for &T {
    fn read_types(types: &mut Vec<TypeId>) {
        types.push(TypeId::of::<T>());
    }

    fn write_types(_: &mut Vec<TypeId>) {}
}

impl<T: Component> QueryAccess for &mut T {
    fn read_types(_: &mut Vec<TypeId>) {}

    fn write_types(types: &mut Vec<TypeId>) {
        types.push(TypeId::of::<T>());
    }
}

impl<A, B> QueryAccess for (A, B)
where
    A: QueryAccess,
    B: QueryAccess,
{
    fn read_types(types: &mut Vec<TypeId>) {
        A::read_types(types);
        B::read_types(types);
    }

    fn write_types(types: &mut Vec<TypeId>) {
        A::write_types(types);
        B::write_types(types);
    }
}

pub(crate) fn validate<Q: QueryAccess>() {
    let mut reads = Vec::new();
    let mut writes = Vec::new();

    Q::read_types(&mut reads);
    Q::write_types(&mut writes);

    for i in 0..writes.len() {
        for j in i + 1..writes.len() {
            assert_ne!(writes[i], writes[j], "Component mutably borrowed twice")
        }
    }

    for read in &reads {
        assert!(
            !writes.contains(read),
            "Component borrowed as immutable and muttable"
        )
    }
}
