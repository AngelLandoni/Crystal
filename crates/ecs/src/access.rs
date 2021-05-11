use std::{
    any::{Any, type_name},
    ops::Deref,
    sync::{Arc, RwLock, RwLockReadGuard},
    marker::PhantomData,
    borrow::Cow
};

use utils::BlockVec;

use crate::{
    entity::Entity,
    component::{Component, ComponentBuffer, BufferBlockVec},
    storage::Storage
};

/// A type that allows read over the component a cross threads.
pub struct Reader<'a, T: 'static + Send + Sync> {
    content: Arc<RwLock<Storage<T>>>,
    _lifetime: PhantomData<&'a ()>
}

impl<'a, T: 'static + Send +Sync> Reader<'a, T> {
    /// Creates and returns a new instance of `Reader`.
    /// 
    /// # Arguments
    /// 
    /// `content` - The content to be referenced.
    fn new(content: Arc<RwLock<Storage<T>>>) -> Self {
        Self {
            content,
            _lifetime: PhantomData
        }
    }
}

/// TODO(Angel): Double check this.
impl<'a, T: 'static + Send + Sync> Reader<'a, T> {
    pub fn read(&self) -> RwLockReadGuard<'_, Storage<T>> {
        self.content.read().unwrap()
    }
}

/// A handy type used to wrap the Lock which contains the storage.
type SLock<R> = RwLock<Storage<R>>;

/// A nice iterator used to walk over the reads.
pub struct ReadAccessIterator<'a, T: 'static + Send + Sync> {
    counter: usize,
    reader: RwLockReadGuard<'a, BufferBlockVec>,
    entities: Arc<Vec<Entity>>,
    _marker: PhantomData<T> 
}

impl<
    'a, T: 'static + Send + Sync
> Iterator for ReadAccessIterator<'a, T> {
    type Item = Reader<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        // Get the entity related with the counter.
        guard!(let Some(entity) = self.entities.get(self.counter) else {
            return None;
        });

        // Check if the item exits if not just return None, that will
        // ends the iterator execution.
        guard!(let Some(item) = self.reader.get(entity.id) else {
            panic!(
                "The entity {} does not contain the component {}",
                123, type_name::<T>()
            );
        });

        // Store a the read in order to keep a reference to it and 
        // avoid borrow checker complains.
        let component = item.read();

        // Get read access over the item.
        guard!(let Ok(c_read) = component else {
            panic!(
                "Error trying to get read access over item at index {}",
                self.counter
            );
        });

        // Get the item itself it it exits otherwise just panic,
        // TODO(Angel): Double check if this can break if the item
        // is deleted in other thread and after that this is read.
        guard!(let Some(u_c_read) = c_read.deref() else {
            panic!(
                "Component {} for entity {} does not exist",
                type_name::<T>(), 123
            );
        });

        let u_c_read_clone = u_c_read.clone();

        // Cast the AnyStorage to the correct type.
        guard!(let Ok(s_ref) = u_c_read_clone.downcast::<SLock<T>>() else {
            panic!(
                "There was a problem trying to cast component to {}",
                type_name::<T>()
            );
        });

        // Increate counter to go to the next entity.
        self.counter += 1;

        Some(Reader::new(s_ref))
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

    fn new(buffer: ComponentBuffer, entities: Arc<Vec<Entity>>) -> Self;
}

/// Provides a type used to read storages from the `World`.
pub struct Read<T: 'static + Send + Sync> {
    buffer: ComponentBuffer,
    entities: Arc<Vec<Entity>>,
    _marker: PhantomData<T> 
}

/// Provieds an Accessible erasure for `Read`.
impl<T: 'static + Send + Sync> Accessible for Read<T> {
    type Component = T;

    fn new(buffer: ComponentBuffer, entities: Arc<Vec<Entity>>) -> Self {
        Self {
            buffer,
            entities,
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
            // Send the correct entities ids.
            entities: self.entities.clone(),
            _marker: PhantomData
        }
    } 
}

pub struct Write<T: 'static + Send + Sync> {
    buffer: ComponentBuffer,
    entities: Arc<Vec<Entity>>,
    _marker: PhantomData<T>
}

/// Provides an Accessible erasure for 'Write'.
impl<T: 'static + Send + Sync> Accessible for Write<T> {
    type Component = T;

    fn new(buffer: ComponentBuffer, entities: Arc<Vec<Entity>>) -> Self {
        Self {
            buffer,
            entities,
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