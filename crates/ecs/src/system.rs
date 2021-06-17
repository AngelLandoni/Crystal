use std::{
    sync::Arc,
    any::type_name
};

use paste::paste;

use crate::{
    bundle::ComponentBundler,
    component::ComponentsHandler,
    access::{Accessible, SLock},
    entity::EntitiesHandler,
    sync::TaskSync,
    type_id::id_of
};

pub trait SystemHandler {
    /// Provides an aftraction used to run a system.
    fn run<
        B: ComponentBundler, S: System<B> + 'static + Send + Sync
    >(&self, system: S) -> Arc<TaskSync>;

    /// Provides an aftraction used to run a system sending a data 
    /// parameter.
    fn run_with_data<
        B: ComponentBundler,
        S: DataSystem<B, D> + 'static + Send + Sync,
        D: 'static + Send
    >(&self, system: S, data: D) -> Arc<TaskSync>;

    /// Provides an aftraction used to run a system sending a data 
    /// parameter that should be exectued in the same thread.
    fn run_sync_with_data<
        'a,
        B: ComponentBundler,
        S: DataSystem<B, D> + 'static + Send + Sync,
        D: 'a + Send
    >(&self, system: S, data: D);
}

pub trait System<B: ComponentBundler> {
    /// Provides an atraction used to execute a system.
    fn run<
        C: ComponentsHandler + Send + Sync,
        E: EntitiesHandler + Send + Sync
    >(self, components_handler: Arc<C>, entities_handler: Arc<E>);
}

pub trait DataSystem<B: ComponentBundler, D: Send> {
    /// Provides an aftraction used to execute a system providing data.
    fn run_with_data<
        C: ComponentsHandler + Send + Sync,
        E: EntitiesHandler + Send + Sync
    >(self, components_handler: Arc<C>, entities_handler: Arc<E>, data: D);
}

impl<F, A> System<(A,)> for F
where 
    F: FnOnce(A) -> (),
    A: 'static + Accessible,
    <A as Accessible>::Component: Sync + Send
{
    fn run<
        C: ComponentsHandler, E: EntitiesHandler
    >(self, components_handler: Arc<C>, entities_handler: Arc<E>) {
        let a_typeid = id_of::<A::Component>();

        let a: A;

        // TODO: Check if we could avoid this using the compiler.
        if A::is_unique() {
            guard!(let Some(c) = components_handler.unique_component(&a_typeid) else {
                panic!(
                    "The component {} does not exist",
                    type_name::<A::Component>()
                );
            });
            guard!(let Ok(c_downcasted) = c.downcast::<SLock<A::Component>>() else {
                panic!("Error casting Arc pointer");
            });
            a = A::unique_new(c_downcasted);
        } else {
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

            a = A::new(a_b, filtered_entities);
        }

        // Create a new instance of Read or Write and and set inside it the
        // reference to the array and send the reference to the block vec.
        (self)(a);
    }
}

macro_rules! generate_system {
    ($($type: ident), +) => {

impl<F, $($type,)+> System<($($type,)+)> for F
where 
    F: FnOnce($($type,)+) -> (),
    $(
        $type: 'static + Accessible,
        <$type as Accessible>::Component: Sync + Send,)+
{
    fn run<
        C: ComponentsHandler + Send + Sync,
        E: EntitiesHandler + Send + Sync
    >(self, components_handler: Arc<C>, entities_handler: Arc<E>) {
        $(
            paste! {
                let [<$type _typeid>] = id_of::<$type::Component>();
                let [<$type _var>]: $type;
            }
        )+

        
        let mut bitmasks = 0x00;

        $(
            if !$type::is_unique() {
                paste! {
                    bitmasks |= components_handler.bitmask([<$type _typeid>]);
                }
            }
        )+
        

        $(
            if $type::is_unique() {
                paste! {
                    guard!(let Some(c) = components_handler.unique_component(&[<$type _typeid>]) else {
                        panic!(
                            "The component {} does not exist",
                            type_name::<A::Component>()
                        );
                    });
                }
                
                paste! {
                    guard!(let Ok(c_downcasted) = c.downcast::<SLock<$type::Component>>() else {
                        panic!("Error casting Arc pointer");
                    });
                    [<$type _var>] = $type::unique_new(c_downcasted);
                }
            } else {
                paste! {
                    // Extract the id of A, in order to get the bitmask.
                    let a_bitmask = components_handler.bitmask([<$type _typeid>]); 
                
                }
               
                // Generate a new buffer with all the entities that matches
                // with this requirement.
                let filtered_entities = Arc::new(
                    entities_handler.query_by_bitmask(bitmasks)
                );

                paste! {
                   // Get the component buffer of a.
                    guard!(let Some(a_b) = components_handler.component_buffer(&[<$type _typeid>]) else {
                        panic!(
                            "The component {} does not exist",
                            type_name::<A::Component>()
                        );
                    }); 
                }
                

                paste! {
                    [<$type _var>] = $type::new(a_b, filtered_entities);
                }
            }
        )+

        (self)(
            $(
                paste! {
                    [<$type _var>]
                }
            ),+
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


impl<F, A, D> DataSystem<(A, ), D> for F
where
    F: FnOnce(D, A) -> (),
    D: 'static + Send,
    A: 'static + Accessible,
    <A as Accessible>::Component: Sync + Send
{
    /// Runs a system providing the data provided by parameter.
    ///
    /// # Arguments
    ///
    /// `components_handler` - The component handler.
    /// `entities_handler` - The entities handler.
    /// `data` - The data to be sent.
    fn run_with_data<
        C: ComponentsHandler + Send + Sync,
        E: EntitiesHandler + Send + Sync
    >(self, components_handler: Arc<C>, entities_handler: Arc<E>, data: D) {
                let a_typeid = id_of::<A::Component>();

        let a: A;

        // TODO: Check if we could avoid this using the compiler.
        if A::is_unique() {
            guard!(let Some(c) = components_handler.unique_component(&a_typeid) else {
                panic!(
                    "The component {} does not exist",
                    type_name::<A::Component>()
                );
            });
            guard!(let Ok(c_downcasted) = c.downcast::<SLock<A::Component>>() else {
                panic!("Error casting Arc pointer");
            });
            a = A::unique_new(c_downcasted);
        } else {
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

            a = A::new(a_b, filtered_entities);
        }

        // Create a new instance of Read or Write and and set inside it the
        // reference to the array and send the reference to the block vec.
        (self)(data, a);
    }
}

macro_rules! generate_data_system {
    ($($type: ident), +) => {

impl<F, $($type,)+ D> DataSystem<($($type,)+), D> for F
where 
    F: FnOnce(D, $($type,)+) -> (),
    D: 'static + Send,
    $(
        $type: 'static + Accessible,
        <$type as Accessible>::Component: Sync + Send,)+
{
    fn run_with_data<
        C: ComponentsHandler + Send + Sync,
        E: EntitiesHandler + Send + Sync
    >(self, components_handler: Arc<C>, entities_handler: Arc<E>, data: D) {
        $(
            paste! {
                let [<$type _typeid>] = id_of::<$type::Component>();
                let [<$type _var>]: $type;
            }
        )+

        
        let mut bitmasks = 0x00;

        $(
            if !$type::is_unique() {
                paste! {
                    bitmasks |= components_handler.bitmask([<$type _typeid>]);
                }
            }
        )+
        

        $(
            if $type::is_unique() {
                paste! {
                    guard!(let Some(c) = components_handler.unique_component(&[<$type _typeid>]) else {
                        panic!(
                            "The component {} does not exist",
                            type_name::<A::Component>()
                        );
                    });
                }
                
                paste! {
                    guard!(let Ok(c_downcasted) = c.downcast::<SLock<$type::Component>>() else {
                        panic!("Error casting Arc pointer");
                    });
                    [<$type _var>] = $type::unique_new(c_downcasted);
                }
            } else {
                paste! {
                    // Extract the id of A, in order to get the bitmask.
                    let a_bitmask = components_handler.bitmask([<$type _typeid>]); 
                
                }
               
                // Generate a new buffer with all the entities that matches
                // with this requirement.
                let filtered_entities = Arc::new(
                    entities_handler.query_by_bitmask(bitmasks)
                );

                paste! {
                   // Get the component buffer of a.
                    guard!(let Some(a_b) = components_handler.component_buffer(&[<$type _typeid>]) else {
                        panic!(
                            "The component {} does not exist",
                            type_name::<A::Component>()
                        );
                    }); 
                }
                

                paste! {
                    [<$type _var>] = $type::new(a_b, filtered_entities);
                }
            }
        )+

        (self)(
            data,
            $(
                paste! {
                    [<$type _var>]
                },
            )+
        );
    }
}

    };
}

generate_data_system!(A, B);
generate_data_system!(A, B, C1);
generate_data_system!(A, B, C1, D1);
generate_data_system!(A, B, C1, D1, E1);
generate_data_system!(A, B, C1, D1, E1, F1);
generate_data_system!(A, B, C1, D1, E1, F1, G);
generate_data_system!(A, B, C1, D1, E1, F1, G, H);
generate_data_system!(A, B, C1, D1, E1, F1, G, H, I);