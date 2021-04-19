use std::{
    iter,
    any::TypeId,
    sync::{Arc, RwLock},
    fmt::{Debug, Result, Formatter},
};

use fxhash::FxHashMap;

use crate::{
    storage::AnyStorage,
    entity::Entity
};

/// Provides an aftraction to handle components.
pub trait ComponentsHandler {
    /// An aftraction used to add a new component into the storage.
    fn add_component<A: 'static + AnyStorage + Send + Sync>(
        &self,
        entity: Entity,
        ids: (TypeId, ),
        component: (A, ));
}

/// Defines a data type that is a reference to the storage, that 
/// reference is thread safe and also implement a Readers and Writers
/// lock.
type ComponentRef = Option<Arc<RwLock<dyn AnyStorage + Send + Sync>>>;

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
    fn add_component<A: 'static + AnyStorage + Send + Sync>(
        &self,
        entity: Entity,
        ids: (TypeId, ),
        component: (A, )) {
        // Take a read lock and check if the component buffer exist.
        let c_reader = self.components.read().unwrap();

        // Check if the key exists, if it does take a reference to the
        // buffer and write over it.
        if let Some(component_buffer) = c_reader.get(&ids.0) {
            // TODO(Angel): Expand the array if it needs more space.

            // Get a reference to the buffer.
            let buffer: ComponentBuffer = component_buffer.clone();
            // Get write lock for the vector. 
            let mut b_writer = buffer.write().unwrap();
            // Replace the current component with a new one.
            // TODO(Angel): Maybe RwLock is not needed here not sure
            // I gonna handle the mut on the future.
            b_writer[entity.id] = Some(Arc::new(RwLock::new(component.0)));
        } else {
            // In order to avoid a deadlock we must drop first the
            // lock on the reader.
            drop(c_reader);
            // At this point we need create a new component buffer
            // due it does not exist.

            // TODO(Angel): N should not be N should be the current size
            // of the needed vector, due a new component could be added
            // in the middle of the execution and that vec should have the 
            // same size as the others.
            let mut new_vec: Vec<ComponentRef> = 
                iter::repeat(None).take(N).collect();
            // Insert the component in the `Entity` spot.
            new_vec[entity.id] = Some(Arc::new(RwLock::new(component.0)));

            // Take a write of the hash map and generate a new 
            // content.
            let mut c_writer = self.components.write().unwrap();
            c_writer.insert(ids.0, Arc::new(RwLock::new(new_vec)));
        }
    }
}

impl<const N: usize> Debug for ComponentsStorage<N> {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        let s_reader = self.components.read().unwrap();
        write!(
            formatter, "number of components: {:?}",
            s_reader.keys().len()
        )
    }
}