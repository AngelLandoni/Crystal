use std::any::TypeId;

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
        // Get more bitmasks.
        
        // Combine bitmasks.
        
        // Create a new instance of Read or Write and and set inside it the
        // reference to the array and send the reference to the block vec.
        let a_access: A = A::new(); 


        // (self)(data, data, data);
    }
}


