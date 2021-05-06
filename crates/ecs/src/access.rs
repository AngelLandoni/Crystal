use std::{
    sync::Arc,
    marker::PhantomData
};

use crate::component::ComponentBuffer;

pub struct AccessIterator<T: 'static + Send + Sync> {
    counter: usize,
    _marker: PhantomData<T> 
}

impl<T: 'static + Send + Sync> Iterator for AccessIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        None 
    }
}

pub trait Accessible: Send + Sync {
    type Component;

    fn new() -> Self;
    fn iter(&self) -> AccessIterator<Self::Component>
        where 
            <Self as Accessible>::Component: Send + Sync; 
}

/// Provides a type used to read storages from the `World`.
pub struct Read<T: 'static + Send + Sync> {
    _marker: PhantomData<T> 
}

/// Provieds an Accessible erasure for `Read`.
impl<T: 'static + Send + Sync> Accessible for Read<T> {
    type Component = T;

    fn new() -> Self {
        Self {
            _marker: PhantomData
        }
    }

    /// Returns a new iterator for `Read`.
    fn iter(&self) -> AccessIterator<Self::Component>
        where 
            <Self as Accessible>::Component: Send + Sync {
        AccessIterator {
            counter: 0,
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

    fn new() -> Self {
        Self {
            _marker: PhantomData
        }
    }

    /// Returns a new iterator for `Write`.
    fn iter(&self) -> AccessIterator<Self::Component>
        where 
            <Self as Accessible>::Component: Send + Sync {
        AccessIterator {
            counter: 0,
            _marker: PhantomData
        }
    } 
}
