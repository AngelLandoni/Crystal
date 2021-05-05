use std::{
    any::{TypeId, type_name},
    sync::{Arc, RwLock},
    fmt::{Debug, Result, Formatter},
};

use fxhash::FxHashMap;
use paste::paste;

use utils::{BlockVec};

use crate::{
    storage::AnyStorage,
    entity::Entity,
    consts::BitmaskType
};

macro_rules! generate_add_component_trait {
    ($name: tt; $([$type: ident, $id: ident]),+) => {
        paste! {
            fn [<add_component $name>]<
                $($type: 'static + AnyStorage + Send + Sync,)+
            >(
                &self,
                entity: Entity,
                ids: ($($id,)+ ),
                component: ($($type,)+ )); 
        }
    };
}

macro_rules! generate_add_component {
    ($name: tt; $([$type: ident, $id: ident, $index: tt]),+) => {
        paste! {
            fn [<add_component $name>]<
                $($type: 'static + AnyStorage + Send + Sync,)+
            >(
                &self,
                entity: Entity,
                ids: ($($id,)+ ),
                component: ($($type,)+ )) {

    // Determines if some of the buffers grow.
    let mut were_expansions: bool = false;

    // In order to avoid a deadlock we must drop first the
    // lock on the reader before sync the buffers (c_reader, b_writer).
    {
        // Take a read lock and check if the component buffer exist.
        let c_reader = self.components.read().unwrap();

        $(
            // Check if the buffer exist if not just panic.
            guard!(let Some(c_buffer) = c_reader.get(&ids.$index) else {
                // The component does not exist, panic an error.
                panic!(
                    "The component {:?} is not registered",
                    type_name::<$type>()
                );
            });

            {
                // Get a reference and write lock to the buffer.
                let buffer: ComponentBuffer<N> = c_buffer.clone();
                let b_reader = buffer.read().unwrap();

                // Check if it can extract the item from the array.
                if let Ok(item) = b_reader.get_inbouds(entity.id) {
                    // If there is memory alrady allocated with a Lock, use
                    // that.
                    if let Some(item_lock) = item {
                        // Get a write over that specific item.
                        let mut i_writer = item_lock.write().unwrap();
                        *i_writer = Some(Arc::new(RwLock::new(component.$index)))
                    // Otherwise create a new entry. 
                    } else {
                        drop(b_reader);
                        were_expansions |=
                            self.add_new_component(&entity, &buffer, component.$index);
                    }
                } else {
                    drop(b_reader);
                    were_expansions |=
                        self.add_new_component(&entity, &buffer, component.$index);
                }
            }
        )+
    }

    // Increate memory of the buffers matching the biggest only
    // if some buffer was expanded.
    if were_expansions {
        self.sync_buffers();
    }

                }
        }
    };
}

pub trait ComponentHandler {
    /// An aftraction used to register one component.
    fn register<C0: 'static>(&self);
}

/// Provides an aftraction to handle components.
pub trait ComponentsHandler {
    /// An aftraction used to register components.
    fn register(&self, c0: TypeId, bitmask_shift: u8);

    /// An aftraction used to add a new component into the storage.
    fn add_component<A: 'static + AnyStorage + Send + Sync>(
        &self,
        entity: Entity,
        ids: (TypeId, ),
        component: (A, ));

    /// An aftraction used to get the associated bitmask.
    fn bitmask(&self, type_id: TypeId) -> BitmaskType;

    /// An aftraction used to remove all the components associated with the
    /// provided entity.
    fn remove_components(&self, entity: &Entity); 

    generate_add_component_trait!(2; [A, TypeId], [B, TypeId]);
    generate_add_component_trait!(3; [A, TypeId], [B, TypeId], [C, TypeId]);
    generate_add_component_trait!(4; [A, TypeId], [B, TypeId], [C, TypeId], [D, TypeId]);
    generate_add_component_trait!(5; [A, TypeId], [B, TypeId], [C, TypeId], [D, TypeId], [E, TypeId]);
    generate_add_component_trait!(6; [A, TypeId], [B, TypeId], [C, TypeId], [D, TypeId], [E, TypeId], [F, TypeId]);
    generate_add_component_trait!(7; [A, TypeId], [B, TypeId], [C, TypeId], [D, TypeId], [E, TypeId], [F, TypeId], [G, TypeId]);
    generate_add_component_trait!(8; [A, TypeId], [B, TypeId], [C, TypeId], [D, TypeId], [E, TypeId], [F, TypeId], [G, TypeId], [H, TypeId]);
    generate_add_component_trait!(9; [A, TypeId], [B, TypeId], [C, TypeId], [D, TypeId], [E, TypeId], [F, TypeId], [G, TypeId], [H, TypeId], [I, TypeId]);
}

type Component = Option<Arc<RwLock<dyn AnyStorage + Send + Sync>>>;

/// Defines a data type that is a reference to the storage.
type ComponentRef = RwLock<Component>;

/// Defines the data structure where the components will be stored.
///
/// RwLock necessary in order to avoid problem when the vec is expanded.
/// 
/// The reference to the Vec must be protected due two or more thread
/// could potentially modify the same index at the same time.
pub(crate) type ComponentBuffer<const N: usize> =
    Arc<RwLock<BlockVec::<ComponentRef, N>>>;

/// Provides an aftraction to store all the components in the ECS.
pub struct ComponentsStorage<const N: usize> {
    /// Contains all the components in the ECS.
    components: RwLock<FxHashMap<TypeId, ComponentBuffer<N>>>,

    /// Contains all the bitmasks of the components.
    bitmasks: RwLock<FxHashMap<TypeId, u8>>,
}

/// Provides default initialization for `ComponentsStorage`.
impl<const N: usize> Default for ComponentsStorage<N> {
    /// Creates and returns a new `ComponentsStorage` with a default
    /// configuration.
    fn default() -> Self {
        Self {
            components: RwLock::new(FxHashMap::default()),
            bitmasks: RwLock::new(FxHashMap::default()),
        }
    }
}

impl<const N: usize> ComponentsHandler for ComponentsStorage<N> {
    /// Registers a component into the `World`.
    fn register(&self, c0: TypeId, bitmask_shift: u8) { 
        {
            // Get exclusive access to the map.
            let mut c_write = self.components.write().unwrap();
            let mut bitmask_c_write = self.bitmasks.write().unwrap();
            // At this point we need create a new component buffer
            // due it does not exist.
            let new_vec = BlockVec::<ComponentRef, N>::new();
            // Insert the new buffer associated with the correct id.
            c_write.insert(c0, Arc::new(RwLock::new(new_vec)));
            // Insert the bitmask shift for the component. 
            bitmask_c_write.insert(c0, bitmask_shift);
        }

        // Sync buffers, this could happen if the component is added
        // after entities.
        self.sync_buffers();        
    }

    /// Removes all the components associated with the provided entity.
    ///
    /// # Arguments
    ///
    /// `entity` - The entity's components to be removed.
    fn remove_components(&self, entity: &Entity) {
        // Take a read lock over the components.
        let c_reader = self.components.read().unwrap();

        // Iterate over each component and erase it.
        // TODO(Angel): Maybe filter by bitmask?.
        for (_, value) in c_reader.iter() {
            let buffer = value.clone();
            let b_reader = buffer.read().unwrap();

            // Check if there is data in the block.
            if let Some(item_ref) = b_reader.get(entity.id).clone() {
                let mut ir_writer = item_ref.write().unwrap();
                *ir_writer = None;
            }
        }
    }

    /// Adds a new component into the storage.
    /// 
    /// In order to write or read to the storage `ComponentsStorage`
    /// use a RwLock, so all the reads are not bloquing but a
    /// write is.
    /// 
    /// If the component does not exist a new buffer will be created
    /// in order to store the content.
    /// 
    /// In order to make this faster register all the component at
    /// the beginning of the binary so the function never tries to 
    /// get a write lock for the Map.
    /// 
    /// # Arguments
    /// 
    /// `entity` - The entity which owns the component.
    /// `ids` - The runtime representation of the provided components.
    /// `components` - The components itself.
    fn add_component<A: 'static + AnyStorage + Send + Sync>(
        &self,
        entity: Entity,
        ids: (TypeId, ),
        component: (A, )) {
        // Determines if some of the buffers grow.
        let mut were_expansions: bool = false;

        // In order to avoid a deadlock we must drop first the
        // lock on the reader before sync the buffers (c_reader, b_writer).
        {
            // Take a read lock and check if the component buffer exist.
            let c_reader = self.components.read().unwrap();

            // Check if the buffer exist if not just panic.
            guard!(let Some(c_buffer) = c_reader.get(&ids.0) else {
                // The component does not exist, panic an error.
                panic!(
                    "The component {:?} is not registered",
                    type_name::<A>()
                );
            });

            // Get a reference and write lock to the buffer.
            let buffer: ComponentBuffer<N> = c_buffer.clone();
            let b_reader = buffer.read().unwrap();

            // Check if it can extract the item from the array.
            if let Ok(item) = b_reader.get_inbouds(entity.id) {
                // If there is memory alrady allocated with a Lock, use
                // that.
                if let Some(item_lock) = item {
                    // Get a write over that specific item.
                    let mut i_writer = item_lock.write().unwrap();
                    *i_writer = Some(Arc::new(RwLock::new(component.0)))
                // Otherwise create a new entry. 
                } else {
                    drop(b_reader);
                    were_expansions |= 
                        self.add_new_component(&entity, &buffer, component.0);
                }
            } else {
                drop(b_reader);
                were_expansions |=
                    self.add_new_component(&entity, &buffer, component.0);
            }
        }

        // Increate memory of the buffers matching the biggest only
        // if some buffer was expanded.
        if were_expansions {
            self.sync_buffers();
        }
    }

    /// Returns the associated bitmask for the `TypeId`.
    ///
    /// # Arguments
    ///
    /// `type_id` - The id used to extract the bitmask.
    fn bitmask(&self, type_id: TypeId) -> BitmaskType {
        // Get read over the bitmasks.
        let b_reader = self.bitmasks.read().unwrap();
        
        // Extract the bitmask if not presset just crash.
        // This should never fail.
        guard!(let Some(shift) = b_reader.get(&type_id) else {
            panic!("The component with id {:?} does not have bitmask", type_id);
        });

        // Generate the bitmask shifting a binary 1 `shift` times.
        0b1 << shift 
    }



    generate_add_component!(2; [A, TypeId, 0], [B, TypeId, 1]);
    generate_add_component!(3; [A, TypeId, 0], [B, TypeId, 1], [C, TypeId, 2]);
    generate_add_component!(4; [A, TypeId, 0], [B, TypeId, 1], [C, TypeId, 2], [D, TypeId, 3]);
    generate_add_component!(5; [A, TypeId, 0], [B, TypeId, 1], [C, TypeId, 2], [D, TypeId, 3], [E, TypeId, 4]);
    generate_add_component!(6; [A, TypeId, 0], [B, TypeId, 1], [C, TypeId, 2], [D, TypeId, 3], [E, TypeId, 4], [F, TypeId, 5]);
    generate_add_component!(7; [A, TypeId, 0], [B, TypeId, 1], [C, TypeId, 2], [D, TypeId, 3], [E, TypeId, 4], [F, TypeId, 5], [G, TypeId, 6]);
    generate_add_component!(8; [A, TypeId, 0], [B, TypeId, 1], [C, TypeId, 2], [D, TypeId, 3], [E, TypeId, 4], [F, TypeId, 5], [G, TypeId, 6], [H, TypeId, 7]);
    generate_add_component!(9; [A, TypeId, 0], [B, TypeId, 1], [C, TypeId, 2], [D, TypeId, 3], [E, TypeId, 4], [F, TypeId, 5], [G, TypeId, 6], [H, TypeId, 7], [I, TypeId, 8]);
}

impl<const N: usize> ComponentsStorage<N> {
    fn sync_buffers(&self) {
        // Get a writer over components in order to avoid 
        // modifications in the buffers sizes in the middle of the 
        // expansion.
        let c_writer = self.components.write().unwrap();

        // Create an vector to store all the write locks.
        let mut writers = Vec::new();

        // Get lock for all the buffers.
        for (_, value) in c_writer.iter() {
            let w = value.write().unwrap();
            writers.push(w);
        }

        // Contains a raw ref to all the blocks, this is safe due
        // we lock all the buffers before, so we have exclusive 
        // access.
         let mut biggest: usize = 0;
    
        // Search for the biggest.
        for w in writers.iter() {
            // Check if the current temp is smaller than the new value.
            if biggest < w.blocks_len() {
                biggest = w.blocks_len();
                continue;
            }
        }

        // Expand the vectors to have the corrent number of blocks.
        for w in writers.iter_mut() {
            let len = w.blocks_len();
            w.append_empty_blocks(biggest - len);
        }
    }

    /// Adds a new component into the world.
    ///
    /// # Arguments
    ///
    /// `entity` - The component owner.
    /// `buffer` - The information.
    /// `components` - The component itself.
    fn add_new_component<C: 'static + AnyStorage + Send + Sync>(
        &self,
        entity: &Entity,
        buffer: &ComponentBuffer<N>,
        component: C) -> bool {
        
        let mut b_writer = buffer.write().unwrap();
        // If there are not items in that position we have to create
        // a new one.

        // Replace the current component with a new one.
        b_writer.set(
            RwLock::new(Some(Arc::new(RwLock::new(component)))),
            entity.id
        )
    }
}

impl<const N: usize> Debug for ComponentsStorage<N> {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        let s_reader = self.components.read().unwrap();
        write!(
            formatter, "number of components: {:?}",
            s_reader.keys().len()
        )
    }
}

