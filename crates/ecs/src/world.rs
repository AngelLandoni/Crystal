use crossbeam_queue::SegQueue;

use std::{
    fmt::{Debug, Result, Formatter},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, RwLock
    }
};

use tasks::{Workers, Task, Dispatcher, Executable};

use crate::{
    type_id::id_of,
    bundle::ComponentBundler,
    component::{
        ComponentsHandler,
        ComponentHandler,
        ComponentsStorage,
        NUM_OF_COMPONETS_PER_PAGE
    },
    entity::{Entity, EntitiesHandler, EntitiesStorage, EntityHandler},
    system::{System, SystemHandler},
    sync::TaskSync
};

/// Defines the size of the entities that should be reached to 
/// allocate the next chunk of data.
/// 
/// To calculate the needed space, PAGE_ENTITY_SIZE * number of pages
/// * number of components * size of a pointer.
type DefaultComponentsStorage = ComponentsStorage;
type DefaultEntitiesStorage = EntitiesStorage::<NUM_OF_COMPONETS_PER_PAGE>;

/// Defines a default `World` wrapper.
pub type DefaultWorld = World<
    DefaultComponentsStorage,
    DefaultEntitiesStorage
>;

pub struct World<
    H: ComponentsHandler + Send + Sync,
    E: EntitiesHandler + Send + Sync
> {
    /// Contains the components storage handler, used to store and 
    /// manage all the components in the `World`.
    components_storage: Arc<H>,

    /// Contains all the entities and the related information to them.
    entities_storage: Arc<E>,
    
    /// Contains a counter of the amount of ids in the `World`. 
    number_of_entities: AtomicUsize,

    /// Contains a counter of the ampunt of components in the `World`.
    number_of_components: AtomicUsize,

    /// Contains a queue of free entities to be used.
    free_entities: SegQueue<Entity>,

    /// Contains the workers pool.
    workers: Workers
}

/// Mark `World` as thread safe.
unsafe impl<
    H: ComponentsHandler + Send + Sync,
    E: EntitiesHandler + Send + Sync
> Send for World<H, E> {}
unsafe impl<
    H: ComponentsHandler + Send + Sync,
    E: EntitiesHandler + Send + Sync
> Sync for World<H, E> {}

impl Default for DefaultWorld {
    /// Creates and returns a new `World` which contains a default
    /// configuration.
    fn default() -> Self {
        // Start workers.
        let mut workers = Workers::default();
        workers.start();

        let c_storage = Arc::new(DefaultComponentsStorage::default());
        let e_storage = Arc::new(DefaultEntitiesStorage::default());

        Self {
            components_storage: c_storage,
            entities_storage: e_storage,
            number_of_entities: AtomicUsize::new(0),
            number_of_components: AtomicUsize::new(0),
            free_entities: SegQueue::new(),
            workers: workers
        }
    }
}

impl<
    H: ComponentsHandler + Send + Sync,
    E: EntitiesHandler + Send + Sync
> EntityHandler for World<H, E> {
    /// Adds a new entity into the `World` with the provided 
    /// components.
    /// 
    /// # Arguments
    /// 
    /// `components` - All the components that the entity contains.
    fn add_entity<B: ComponentBundler>(
        &self,
        components: B) -> Entity {
        // Increate the number of components.
        self.number_of_components.fetch_add(
            components.len(), Ordering::SeqCst);
        
        // Generate a new entity. For now we are not reusing entities
        // so as soon as this thing is finished we have to do a pool
        // of not used entities.
        let entity: Entity = self.generate_entity();
            
        // Add all the components to the entity.
        let bitmask = 
            components.add_components(
                entity, self.components_storage.clone());

        // Register the bitmask for the given entity.
        self.entities_storage.register_bitmask(&entity, &bitmask);
        
        entity
    }

    /// Removes an entity from the `World`.
    /// 
    /// # Arguments
    /// 
    /// `entity` - The entity to be deleted.
    fn remove_entity(&self, entity: Entity) {
        self.entities_storage.reset_bitmask(&entity);
        self.components_storage.remove_components(&entity);

        // Add move entity to the pool.
        self.free_entities.push(entity);
    }
}

impl<
    H: ComponentsHandler + Send + Sync,
    E: EntitiesHandler + Send + Sync
> ComponentHandler for World<H, E> {
    /// Registers a new component into the system.
    fn register<C0: 'static>(&self) {
        // Generate an unique id for the component.
        let id = id_of::<C0>();
        let bm_shift = self.number_of_components.fetch_add(1, Ordering::SeqCst);
        // Register the component.
        self.components_storage.register(id, bm_shift as u8);
    }
}

/// Provide handy functions.
impl<
    H: ComponentsHandler + Send + Sync,
    E: EntitiesHandler + Send + Sync
> World<H, E> {
    /// Generates and returns a new `Entity`.
    ///
    /// If there is an avaialbe id not used that will be reused.
    fn generate_entity(&self) -> Entity {
        if let Some(free_entity) = self.free_entities.pop() {
            return free_entity;
        }

        Entity::new(self.number_of_entities.fetch_add(1, Ordering::SeqCst)) 
   }  
}

/// Provides handy functions to handle the systems.
impl<
    H: ComponentsHandler + Send + Sync + 'static,
    E: EntitiesHandler + Send + Sync + 'static
> SystemHandler for World<H, E> {
    fn run<
        B: ComponentBundler, Sys: System<B> + 'static + Send + Sync
    >(&self, system: Sys) -> Arc<TaskSync> {
        // Get a clone of the storages in order to send them to the
        // queue.
        let c_s_copy = self.components_storage.clone();
        let e_s_copy = self.entities_storage.clone();

        // Generate a signal in order to know when the task finish.
        let task_sync = Arc::new(TaskSync::default());
        let task_sync_copy = task_sync.clone();

        // This must by run in a worker thread.
        self.workers.execute_dyn(Box::new(move || {
            system.run(c_s_copy, e_s_copy);
            task_sync_copy.mark_as_finish();
        }));

        task_sync
    }
}

impl<
    H: ComponentsHandler + Send + Sync,
    E: EntitiesHandler + Send + Sync
> Debug for World<H, E> {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(
            formatter, "number of entities: {:?}", // | {:?}",
            self.number_of_entities,
            //self.components_storage
        )
    }
}

