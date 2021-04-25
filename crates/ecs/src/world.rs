use std::{
    fmt::{Debug, Result, Formatter},
    sync::atomic::{AtomicUsize, Ordering}
};

use crate::{
    type_id::id_of,
    bundle::ComponentBundler,
    component::{
        ComponentsHandler,
        ComponentHandler,
        ComponentsStorage
    },
    entity::{Entity, EntityHandler},
};

/// Defines the size of the entities that should be reached to 
/// allocate the next chunk of data.
/// 
/// To calculate the needed space, PAGE_ENTITY_SIZE * number of pages
/// * number of components * size of a pointer.
const PAGE_ENTITY_SIZE: usize = 400;
type DefaultComponentsStorage = ComponentsStorage::<PAGE_ENTITY_SIZE>;

/// Defines a default `World` wrapper.
pub type DefaultWorld = World<DefaultComponentsStorage>;

/// TODO(Angel): Entity pool.
/// TODO(Angel): Entity bitmask.
pub struct World<H: ComponentsHandler> {
    /// Contains the components storage handler, used to store and 
    /// manage all the components in the `World`.
    storage: H,
    
    /// Contains a counter of the amount of ids in the `World`. 
    number_of_entities: AtomicUsize
}

impl Default for World<DefaultComponentsStorage> {
    /// Creates and returns a new `World` which contains a default
    /// configuration.
    fn default() -> Self {
        Self {
            storage: DefaultComponentsStorage::default(),
            number_of_entities: AtomicUsize::new(0)
        }
    }
}

impl<H: ComponentsHandler> EntityHandler for World<H> {
    /// Adds a new entity into the `World` with the provided 
    /// components.
    /// 
    /// # Arguments
    /// 
    /// `components` - All the components that the entity contains.
    fn add_entity<B: ComponentBundler>(
        &self,
        components: B) -> Entity {
        // Generate a new entity. For now we are not reusing entities
        // so as soon as this thing is finished we have to do a pool
        // of not used entities.
        let id = self.number_of_entities.fetch_add(
            1,
            Ordering::SeqCst
        );

        // Create a new entity using the thread safe id.
        let entity = Entity::new(id);

        // Add all the components to the entity.
        components.add_components(entity, &self.storage);
        
        entity
    }

    fn remove_entity(&mut self, entity: Entity) {

    }
}

impl<H: ComponentsHandler> ComponentHandler for World<H> {
    /// Registers a new component into the system.
    fn register<C0: 'static>(&self) {
        // Generate an unique id for the component.
        let id = id_of::<C0>();
        // Register the component.
        self.storage.register(id);
    }
}

impl<H: ComponentsHandler + Debug> Debug for World<H> {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(
            formatter, "number of entities: {:?} | {:?}",
            self.number_of_entities,
            self.storage
        )
    }
}