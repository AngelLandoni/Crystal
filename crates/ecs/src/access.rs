use std::marker::PhantomData;

use std::iter::Iterator;

pub trait Accessible: Send + Sync {
    type Component;

    /// Should return a new instance of the type.
    fn new() -> Self;
}

/// Provides a type used to read storages from the `World`.
pub struct Read<T: 'static + Send + Sync> {
    _marker: PhantomData<T> 
}

/// Provieds an Accessible erasure for `Read`.
impl<T: 'static + Send + Sync> Accessible for Read<T> {
    type Component = T;

    /// Creates and returns a new instance of Read.
    fn new() -> Self {
        Self {
            _marker: PhantomData
        }
    }
}  

pub struct Write<T: 'static + Send + Sync> {
    _marker: PhantomData<T>
}

/// Provides an Accessible erasure for 'Write'.
impl<T: 'static + Send + Sync> Accessible for Write<T> {
    type Component = T;

    /// Creates and returns a new instance of Write.
    fn new() -> Self {
        Self {
            _marker: PhantomData
        }
    }
}
