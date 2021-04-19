use std::{
    any::TypeId,
    sync::{Arc, RwLock}
};

use fxhash::FxHashMap;

use crate::{
    storage::AnyStorage,
    entity::Entity
};

/// Provides an aftraction to handle components.
pub trait ComponentsHandler {
    /// An aftraction used to add a new component into the storage.
    fn add_component<A: 'static + AnyStorage>(
        &self,
        entity: Entity,
        ids: (TypeId, ),
        component: (A, ));
}

/// Defines a data type that is a reference to the storage, that 
/// reference is thread safe and also implement a Readers and Writers
/// lock.
type ComponentRef = Arc<RwLock<dyn AnyStorage>>;

/// Defines the data structure where the components will be stored.
/// 
/// The reference to the Vec must be protected due two or more thread
/// could potentially modify the same index at the same time.
type ComponentBuffer = Arc<RwLock<Vec<ComponentRef>>>;

/// Provides an aftraction to store all the components in the ECS.
pub struct ComponentsStorage<const N: usize> {
    /// Contains all the components in the ECS.
    components: RwLock<FxHashMap<TypeId, ComponentBuffer>>
}

/// Provides default initialization for `ComponentsStorage`.
impl<const N: usize> Default for ComponentsStorage<N> {
    /// Creates and returns a new `ComponentsStorage` with a default
    /// configuration.
    fn default() -> Self {
        Self {
            components: RwLock::new(FxHashMap::default())
        }
    }
}

impl<const N: usize> ComponentsHandler for ComponentsStorage<N> {
    /// Adds a new component into the storage.
    /// 
    /// In order to write or read to the storage `ComponentsStorage`
    /// use a RwLock, so all the reads are not bloquing but a
    /// write is.
    /// 
    /// If the component does not exist a new buffer will be created
    /// in order to store the content.
    /// 
    /// In order to make this faster register all the component at
    /// the beginning of the binary so the function never tries to 
    /// get a write lock for the Map.
    /// 
    /// # Arguments
    /// 
    /// `entity` - The entity which owns the component.
    /// `ids` - The runtime representation of the provided components.
    /// `components` - The components itself.
    fn add_component<A: 'static + AnyStorage>(
        &self,
        entity: Entity,
        ids: (TypeId, ),
        component: (A, )) {
        // Take a read lock and check if the component buffer exist.
        let components = self.components.read().unwrap();

        // Check if the key exists, if it does take a reference to the
        // buffer and write over it.
        if let Some(component_buffer) = components.get(&ids.0) {
            // Get a reference to the buffer.
            let buffer: ComponentBuffer = component_buffer.clone();
            // Get write lock for the vector. 
            let mut b_writter = buffer.write().unwrap();
            // Replace the current component with a new one. 
            b_writter[entity.id] = Arc::new(RwLock::new(component.0));
        }
    }
}