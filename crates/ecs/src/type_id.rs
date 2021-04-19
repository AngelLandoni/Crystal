use std::any::TypeId;

/// Creates and returns an id representation of the provided type.
pub(crate) fn id_of<T: ?Sized + 'static>() -> TypeId {
    TypeId::of::<T>()
}