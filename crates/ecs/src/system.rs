use std::{
    borrow::Cow,
    sync::Arc,
    any::type_name
};

use paste::paste;

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

macro_rules! generate_system {
    ($($type: ident), +) => {

impl<F, $($type,)+> System<($($type,)+)> for F
where 
    F: FnOnce($($type,)+) -> (),
    $($type: 'static + Accessible,)+
{
    fn run<
        C: ComponentsHandler, E: EntitiesHandler
    >(self, components_handler: &C, entities_handler: &E) {
        $(
            paste! {
                let [<$type _typeid>] = id_of::<$type::Component>();
                // Extract the id of A, in order to get the bitmask.
                let [<$type _bitmask>] = components_handler.bitmask([<$type _typeid>]); 
                
            }
        )+

        // Generate a new buffer with all the entities that matches
        // with this requirement.
        let filtered_entities = Arc::new(
            entities_handler.query_by_bitmask(
                $(
                    paste! {
                        [<$type _bitmask>]
                    } |
                )+ 
                0x00               
            )
        );

        $(
            paste! {
                // Get the component buffer of a.
                guard!(let Some([<$type _b>]) = components_handler.component_buffer(&[<$type _typeid>]) else {
                    panic!(
                        "The component {} does not exist",
                        type_name::<$type::Component>()
                    );
                });
            }
        )+

        // Create a new instance of Read or Write and and set inside it the
        // reference to the array and send the reference to the block vec.
        (self)(
            $(
                paste! {
                    $type::new([<$type _b>], filtered_entities.clone())
                },
            )+
        );
    }
}

    };
}

generate_system!(A, B);
generate_system!(A, B, C1);
generate_system!(A, B, C1, D);
generate_system!(A, B, C1, D, E1);
generate_system!(A, B, C1, D, E1, F1);
generate_system!(A, B, C1, D, E1, F1, G);
generate_system!(A, B, C1, D, E1, F1, G, H);
generate_system!(A, B, C1, D, E1, F1, G, H, I);