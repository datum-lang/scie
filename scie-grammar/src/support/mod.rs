use std::any::{Any, TypeId};

pub fn get_type_of<T: ?Sized + Any>(_s: &T) -> TypeId {
    TypeId::of::<T>()
}
