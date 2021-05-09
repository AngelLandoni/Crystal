use std::{
    any::Any,
    ops::Deref,
    sync::{Arc, RwLock, RwLockReadGuard},
    marker::PhantomData
};

use utils::BlockVec;

use crate::{
    component::{Component, ComponentBuffer, BufferBlockVec},
    storage::Storage
};

pub struct ReadAccessIterator<'a, T: 'static + Send + Sync> {
    counter: usize,
    reader: RwLockReadGuard<'a, BufferBlockVec>,
    _marker: PhantomData<T> 
}

impl<
    'a, T: 'static + Send + Sync
> Iterator for ReadAccessIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.reader.get(self.counter) {
            // Get read lock over the component ref.
            if let Some(component_read) = item.read().unwrap().deref() {
                let component_copy = component_read.clone();
                if let Some(storage) = component_copy.downcast::<RwLock<Storage<T>>>() {

                }
            }

            // Check if the item exist, get a copy of of the reference
            // and a read to the internal component itself.
            /*if let Some(i_ref) = component_read {
                // Get a copy reference to the item.
                let i_copy = i_ref.clone();
                // Get a read to that clone.
            }
            let the_data = i_read as Self::Item;*/
        }

        None
    }
}

pub struct WriteAccessIterator<T: 'static + Send + Sync> {
    counter: usize,
    buffer: ComponentBuffer,
    _marker: PhantomData<T> 
}

impl<T: 'static + Send + Sync> Iterator for WriteAccessIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

pub trait Accessible: Send + Sync {
    type Component;

    fn new(buffer: ComponentBuffer) -> Self;
}

/// Provides a type used to read storages from the `World`.
pub struct Read<T: 'static + Send + Sync> {
    buffer: ComponentBuffer,
    _marker: PhantomData<T> 
}

/// Provieds an Accessible erasure for `Read`.
impl<T: 'static + Send + Sync> Accessible for Read<T> {
    type Component = T;

    fn new(buffer: ComponentBuffer) -> Self {
        Self {
            buffer, 
            _marker: PhantomData
        }
    }
}

impl<T: 'static + Send + Sync> Read<T> { 
    /// Returns a new iterator for `Read`.
    pub fn iter(&self) -> ReadAccessIterator<T>
        where 
            <Self as Accessible>::Component: Send + Sync {
        ReadAccessIterator {
            counter: 0,
            // Take a read access now to avoid multiples read access when
            // the iterator loops
            reader: self.buffer.read().unwrap(),
            _marker: PhantomData
        }
    } 
}

pub struct Write<T: 'static + Send + Sync> {
    buffer: ComponentBuffer,
    _marker: PhantomData<T>
}

/// Provides an Accessible erasure for 'Write'.
impl<T: 'static + Send + Sync> Accessible for Write<T> {
    type Component = T;

    fn new(buffer: ComponentBuffer) -> Self {
        Self {
            buffer,
            _marker: PhantomData
        }
    }
}

impl<T: 'static + Send + Sync> Write<T> { 
    /// Returns a new iterator for `Read`.
    pub fn iter(&self) -> WriteAccessIterator<T>
        where 
            <Self as Accessible>::Component: Send + Sync {
        WriteAccessIterator {
            counter: 0,
            buffer: self.buffer.clone(),
            _marker: PhantomData
        }
    } 
}