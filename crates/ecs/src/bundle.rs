use std::any::TypeId;

use crate::{
    type_id::id_of,
    component::ComponentsHandler,
    entity::Entity,
    storage::Storage
};

/// Provides an aftraction used to add components into a handler.
pub trait ComponentBundler {
    /// An aftraction used to add components into the handler.
    fn add_components<H: ComponentsHandler>(
        self,
        entity: Entity,
        handler: &H);
}

impl<T: 'static + Send + Sync> ComponentBundler for (T, ) {
    /// Adds a new component for the provided `Entity`.
    /// 
    /// # Arguments
    /// 
    /// `entity` - The entity which receives the component.
    /// `handler` - Where the component will be stored.
    fn add_components<H: ComponentsHandler>(
        self,
        entity: Entity,
        handler: &H) {

        // Get the type id of the first element in the tuple.
        let a_id: TypeId = id_of::<T>();
        // Create a new storage and safe the data there.
        let a_storage = Storage::new(self.0);

        // Send the component to the handler.
        handler.add_component(entity, (a_id, ), (a_storage, ));
    }
}