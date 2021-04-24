use std::{
    iter,
    any::TypeId,
    sync::{Arc, RwLock, RwLockWriteGuard},
    fmt::{Debug, Result, Formatter},
};

use fxhash::FxHashMap;

use utils::{BlockVec, sync_mem_to_biggest};

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
type ComponentRef = Arc<RwLock<dyn AnyStorage + Send + Sync>>;

/// Defines the data structure where the components will be stored.
/// 
/// The reference to the Vec must be protected due two or more thread
/// could potentially modify the same index at the same time.
type ComponentBuffer = Arc<RwLock<BlockVec::<ComponentRef, 400>>>;

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
        // Determines if some of the buffers grow.
        let mut were_expansions: bool = false;

        // In order to avoid a deadlock we must drop first the
        // lock on the reader before sync the buffers (c_reader, b_writer).
        {
            // Take a read lock and check if the component buffer exist.
            let c_reader = self.components.read().unwrap();

            // Check if the buffer exist if not just panic.
            guard!(let Some(c_buffer) = c_reader.get(&ids.0) else {
                // The component does not exist, panic an error.
                panic!("The component {:?} is not registered", ids.0);
            });

            // Get a reference and write lock to the buffer.
            let buffer: ComponentBuffer = c_buffer.clone();
            let mut b_writer = buffer.write().unwrap();

            // Replace the current component with a new one.
            were_expansions |= b_writer.set(
                Arc::new(RwLock::new(component.0)),
                entity.id
            );
        }

        // Increate memory of the buffers matching the biggest only
        // if some buffer was expanded.
        if were_expansions {
            self.sync_buffers();
        }
    }
}

impl<const N: usize> ComponentsStorage<N> {
    fn sync_buffers<'a>(&self) {
        // Get a writer over components in order to avoid 
        // modifications in the buffers sizes in the middle of the 
        // expansion.
        let c_writer = self.components.write().unwrap();

        // Create an vector to store all the write locks.
        let mut writers = Vec::new();

        // Get lock for all the buffers.
        for (_, value) in c_writer.iter() {
            let w = value.write().unwrap();
            writers.push(w);
        }

        // Contains a raw ref to all the blocks, this is safe due
        // we lock all the buffers before, so we have exclusive 
        // access.
         let mut biggest: usize = 0;
    
        // Search for the biggest.
        for w in writers.iter() {
            // Check if the current temp is smaller than the new value.
            if biggest < w.blocks_len() {
                biggest = w.blocks_len();
                continue;
            }
        }

        // Expand the vectors to have the corrent number of blocks.
        for w in writers.iter_mut() {
            let len = w.blocks_len();
            w.append_empty_blocks(biggest - len);
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


/*else {
            // At this point we need create a new component buffer
            // due it does not exist.
            let mut new_vec = BlockVec::<ComponentRef, 400>::new();
            new_vec.set(
                Some(Arc::new(RwLock::new(component.0))),
                entity.id
            );

            // In order to avoid a deadlock we must drop first the
            // lock on the reader.
            drop(c_reader);
            // Take a write of the hash map and generate a new 
            // content.
            let mut c_writer = self.components.write().unwrap();
            c_writer.insert(ids.0, Arc::new(RwLock::new(new_vec)));
           
            drop(c_writer);
            // Increate memory of the buffers matching the biggest.
            self.sync_buffers();
        }*/