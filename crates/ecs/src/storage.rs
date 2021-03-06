use std::ops::{Deref, DerefMut};

/// TODO(Angel): This thing should go away.
pub trait AnyStorage {}

/// A wrapper over the components that allow us avoid force the
/// dev to implement a trait over their own components.
pub struct Storage<T> {
    /// The component itself.
    component: T,
}

impl<T> Storage<T> {
    /// Creates and returns a new storage which contains the provided
    /// component.
    pub fn new(component: T) -> Self {
        Self { component }
    }
}

impl<T> Deref for Storage<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.component
    }
}

impl<T> DerefMut for Storage<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.component
    }
}

impl<T> AnyStorage for Storage<T> {}
