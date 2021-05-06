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
    fn run<H: ComponentsHandler<N>, const N: usize>(self, handler: &H);
}

impl<F, A> System<(A,)> for F
where 
    F: FnOnce(A) -> (),
    A: 'static + Accessible
{
    fn run<H: ComponentsHandler<N>, const N: usize>(self, handler: &H) {
        let a_typeid = id_of::<A::Component>();
        // Extract the id of A, in order to get the bitmask.
        let a_bitmask = handler.bitmask(a_typeid); 
        // Get the component buffer of a.
        let ref_to_vec = handler.component_buffer(&a_typeid);
        
        // Create a new instance of Read or Write and and set inside it the
        // reference to the array and send the reference to the block vec.
        //let a_access: A = A::new(); 
        (self)(A::new());
    }
}


