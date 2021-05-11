use std::{
    borrow::Cow,
    sync::Arc,
    any::type_name
};

use crate::{
    bundle::ComponentBundler,
    component::ComponentsHandler,
    access::Accessible,
    entity::EntitiesHandler,
    type_id::id_of
};

pub trait SystemHandler {
    fn run<B: ComponentBundler, S: System<B>>(&self, system: S);
}

pub trait System<B: ComponentBundler> {
    fn run<
        C: ComponentsHandler, E: EntitiesHandler
    >(self, components_handler: &C, entities_handler: &E);
}

impl<F, A> System<(A,)> for F
where 
    F: FnOnce(A) -> (),
    A: 'static + Accessible
{
    fn run<
        C: ComponentsHandler, E: EntitiesHandler
    >(self, components_handler: &C, entities_handler: &E) {
        let a_typeid = id_of::<A::Component>();
        // Extract the id of A, in order to get the bitmask.
        let a_bitmask = components_handler.bitmask(a_typeid); 
        
        // Generate a new buffer with all the entities that matches
        // with this requirement.
        let filtered_entities = Arc::new(
            entities_handler.query_by_bitmask(a_bitmask)
        );

        // Get the component buffer of a.
        guard!(let Some(a_b) = components_handler.component_buffer(&a_typeid) else {
            panic!(
                "The component {} does not exist",
                type_name::<A::Component>()
            );
        });
        
        // Create a new instance of Read or Write and and set inside it the
        // reference to the array and send the reference to the block vec.
        (self)(A::new(a_b, filtered_entities));
    }
}


