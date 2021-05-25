use std::{
    any::type_name,
    ops::Deref,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
    marker::PhantomData,
};

use crate::{
    entity::Entity,
    component::{ComponentBuffer, BufferBlockVec, UniqueComponent},
    storage::Storage
};

pub trait Accessible: Send + Sync {
    type Component;

    fn new(buffer: ComponentBuffer, entities: Arc<Vec<Entity>>) -> Self;
    fn unique_new(component: Arc<SLock<Self::Component>>) -> Self;

    fn is_unique() -> bool;
}

/// Read access.

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

impl<'a, T: 'static + Send + Sync> Reader<'a, T> {
    pub fn read(&self) -> RwLockReadGuard<'_, Storage<T>> {
        self.content.read().unwrap()
    }
}

/// A handy type used to wrap the Lock which contains the storage.
pub(crate) type SLock<R> = RwLock<Storage<R>>;

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
        // Loosing lock access?.

        // Increate counter to go to the next entity.
        self.counter += 1;

        Some(Reader::new(s_ref))
    }
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

    /// This function is not available for the Read type.
    fn unique_new(_component: Arc<SLock<Self::Component>>) -> Self {
        panic!("unique_new is not available for Read");
    }

    fn is_unique() -> bool { false }
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

/// Write access

/// A type that allows write over the component a cross threads.
pub struct Writter<'a, T: 'static + Send + Sync> {
    content: Arc<RwLock<Storage<T>>>,
    _lifetime: PhantomData<&'a ()>
}

impl<'a, T: 'static + Send + Sync> Writter<'a, T> {
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

    /// This function is not available for the Read type.
    fn unique_new(_unique: UniqueComponent) -> Self {
        panic!("unique_new is not available for Read");
    }
}

impl<'a, T: 'static + Send + Sync> Writter<'a, T> {
    pub fn write(&self) -> RwLockWriteGuard<'_, Storage<T>> {
        self.content.write().unwrap()
    }
}

/// A nice iterator used to wrape the lock which contains the storage.
pub struct WriteAccessIterator<'a, T: 'static + Send + Sync> {
    counter: usize,
    reader: RwLockReadGuard<'a, BufferBlockVec>,
    entities: Arc<Vec<Entity>>,
    _marker: PhantomData<T>  
}

impl<
    'a, T: 'static + Send + Sync
> Iterator for WriteAccessIterator<'a, T> {
    type Item = Writter<'a, T>;

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
        let component = item.write();

        // Get read access over the item.
        guard!(let Ok(c_write) = component else {
            panic!(
                "Error trying to get read access over item at index {}",
                self.counter
            );
        });

        // Get the item itself it it exits otherwise just panic,
        // TODO(Angel): Double check if this can break if the item
        // is deleted in other thread and after that this is read.
        guard!(let Some(u_c_write) = c_write.deref() else {
            panic!(
                "Component {} for entity {} does not exist",
                type_name::<T>(), 123
            );
        });

        let u_c_write_clone = u_c_write.clone();

        // Cast the AnyStorage to the correct type.
        guard!(let Ok(s_ref) = u_c_write_clone.downcast::<SLock<T>>() else {
            panic!(
                "There was a problem trying to cast component to {}",
                type_name::<T>()
            );
        });

        // Increate counter to go to the next entity.
        self.counter += 1;

        Some(Writter::new(s_ref))
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

    /// This function is not available for the Read type.
    fn unique_new(_component: Arc<SLock<Self::Component>>) -> Self {
        panic!("unique_new is not available for Read");
    }

    fn is_unique() -> bool { false }
}

impl<T: 'static + Send + Sync> Write<T> { 
    /// Returns a new iterator for `Read`.
    pub fn iter(&self) -> WriteAccessIterator<T>
        where 
            <Self as Accessible>::Component: Send + Sync {
        WriteAccessIterator {
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

/// Defines a data type which allows the user access a unique type in the 
/// `World`.
pub struct UniqueRead<T: 'static + Send + Sync> {
    /// A container for the component ref.
    unique: Arc<SLock<T>>,

    /// Phantom data need in order to keep the T.
    _marker: PhantomData<T>
}

impl<T: 'static + Send + Sync> Accessible for UniqueRead<T> {
    type Component = T;

    fn new(_buffer: ComponentBuffer, _entities: Arc<Vec<Entity>>) -> Self {
        panic!("new is not available for UniqueRead try with unique_new");
    }

    /// This function is not available for the Read type.
    fn unique_new(component: Arc<SLock<T>>) -> Self {
        Self {
            unique: component,
            _marker: PhantomData
        }
    }

    fn is_unique() -> bool { true }
}

impl<T: 'static + Send + Sync> UniqueRead<T> {
    pub fn read(&self) -> RwLockReadGuard<'_, Storage<T>> {
        self.unique.read().unwrap()
    }
}