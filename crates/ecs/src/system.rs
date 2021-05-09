use std::any::type_name;

use crate::{
    bundle::ComponentBundler,
    component::ComponentsHandler,
    access::Accessible,
    type_id::id_of
};

pub trait SystemHandler {
    fn run<B: ComponentBundler, S: System<B>>(&self, system: S);
}

pub trait System<B: ComponentBundler> {
    fn run<H: ComponentsHandler>(self, handler: &H);
}

impl<F, A> System<(A,)> for F
where 
    F: FnOnce(A) -> (),
    A: 'static + Accessible
{
    fn run<H: ComponentsHandler>(self, handler: &H) {
        let a_typeid = id_of::<A::Component>();
        // Extract the id of A, in order to get the bitmask.
        let a_bitmask = handler.bitmask(a_typeid); 
        
        // Get the component buffer of a.
        guard!(let Some(a_b) = handler.component_buffer(&a_typeid) else {
            panic!(
                "The component {} does not exist",
                type_name::<A::Component>()
            );
        });
        
        // Create a new instance of Read or Write and and set inside it the
        // reference to the array and send the reference to the block vec.
        (self)(A::new(a_b));
    }
}


