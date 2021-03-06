use std::{
    sync::Arc,
    any::TypeId
};

use paste::paste;

use crate::{
    type_id::id_of,
    component::ComponentsHandler,
    entity::Entity,
    storage::Storage,
    consts::BitmaskType
};

/// Provides an aftraction used to add components into a handler.
pub trait ComponentBundler {
    /// An aftraction used to add components into the handler.
    fn add_components<Z: ComponentsHandler + Send + Sync>(
        self,
        entity: Entity,
        handler: Arc<Z>) -> BitmaskType;

    /// An aftraction used to return the number of component in 
    /// the bundle.
    fn len(&self) -> usize;
}

impl<T: 'static + Send + Sync> ComponentBundler for (T, ) {
    /// Adds a new component for the provided `Entity`.
    /// 
    /// # Arguments
    /// 
    /// `entity` - The entity which receives the component.
    /// `handler` - Where the component will be stored.
    fn add_components<Z: ComponentsHandler + Send + Sync>(
        self,
        entity: Entity,
        handler: Arc<Z>) -> BitmaskType {

        // Get the type id of the first element in the tuple.
        let a_id: TypeId = id_of::<T>();
        // Create a new storage and safe the data there.
        let a_storage = Storage::new(self.0);

        // Send the component to the handler.
        handler.add_component(entity, (a_id, ), (a_storage, ));
    
        // Generate the bitmask.
        handler.bitmask(a_id) 
    }

    /// Returns the number of components.
    fn len(&self) -> usize { 1 }
}

macro_rules! generate_bundle {
    ($name: tt; $([$type: ident, $index: tt]), +) => {
        impl<
            $($type: 'static + Send + Sync,)+
        > ComponentBundler for ($($type), +) {
            fn add_components<
                Z: ComponentsHandler + Send + Sync
            >(self, entity: Entity, handler: Arc<Z>) -> BitmaskType {
                paste! {
                    handler.[<add_component $name>](
                        entity,
                        ($(id_of::<$type>(),)+),
                        ($(Storage::new(self.$index),)+)
                    )
                }

                // In order to avoid the last & (not sure how to get ride of
                // it) we use a constant with all 1 to avoid any change
                // over the real bitmask.
                $(
                    handler.bitmask(id_of::<$type>()) |
                )+ 0x0
            }

            fn len(&self) -> usize {
                $name
            }
        }
    };
}

generate_bundle!(2; [A, 0], [B, 1]);
generate_bundle!(3; [A, 0], [B, 1], [C, 2]);
generate_bundle!(4; [A, 0], [B, 1], [C, 2], [D, 3]);
generate_bundle!(5; [A, 0], [B, 1], [C, 2], [D, 3], [E, 4]);
generate_bundle!(6; [A, 0], [B, 1], [C, 2], [D, 3], [E, 4], [F, 5]);
generate_bundle!(7; [A, 0], [B, 1], [C, 2], [D, 3], [E, 4], [F, 5], [G, 6]);
generate_bundle!(8; [A, 0], [B, 1], [C, 2], [D, 3], [E, 4], [F, 5], [G, 6], [H, 7]);
generate_bundle!(9; [A, 0], [B, 1], [C, 2], [D, 3], [E, 4], [F, 5], [G, 6], [H, 7], [I, 8]);
