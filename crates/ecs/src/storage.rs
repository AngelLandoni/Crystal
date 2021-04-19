pub trait AnyStorage {
    // Method to return the component, any idea how to cast that 
    // to the correct value?.
}

/// A wrapper over the components that allow us avoid force the
/// dev to implement a trait over their own components.
pub struct Storage<T> {
    /// The component itself.
    component: T
}

impl<T> Storage<T> {
    /// Creates and returns a new storage which contains the provided
    /// component.
    pub(crate) fn new(component: T) -> Self {
        Self {
            component
        }
    }
}

impl<T> AnyStorage for Storage<T> {}