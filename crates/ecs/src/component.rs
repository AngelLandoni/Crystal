use std::{
    any::{type_name, Any, TypeId},
    fmt::{Debug, Formatter, Result},
    sync::{Arc, RwLock},
};

use fxhash::FxHashMap;
use paste::paste;

use utils::BlockVec;

use crate::{
    access::Accessible,
    consts::BitmaskType,
    entity::Entity,
    storage::AnyStorage,
    storage::Storage
};

/// Defines the number of componets per page in the block vec.
pub(crate) const NUM_OF_COMPONETS_PER_PAGE: usize = 400;

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
                let buffer: ComponentBuffer = c_buffer.clone();
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

    /// An aftraction used to register unique components.
    fn register_unique<C0: 'static + Send + Sync>(&self, c: C0);
}

/// Provides an aftraction to handle components.
pub trait ComponentsHandler {
    /// An aftraction used to register components.
    fn register(&self, c0: TypeId, bitmask_shift: u8);

    /// An aftraction used to register a unique component.
    fn register_unique<C0: 'static + Send + Sync>(&self, id: TypeId, c: C0);

    /// An aftraction used to add a new component into the storage.
    fn add_component<A: 'static + AnyStorage + Send + Sync>(
        &self,
        entity: Entity,
        ids: (TypeId,),
        component: (A,),
    );

    /// An aftraction used to get the associated bitmask.
    fn bitmask(&self, type_id: TypeId) -> BitmaskType;

    /// An aftraction used to return the component buffer for a specific type.
    fn component_buffer(&self, type_id: &TypeId) -> Option<ComponentBuffer>;

    /// An aftraction used to return the component for the type id.
    fn unique_component(&self, type_id: &TypeId) -> Option<UniqueComponent>;

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

pub(crate) type Component = Option<Arc<dyn Any + Send + Sync>>;

/// Defines a data type that is a reference to the storage.
type ComponentRef = RwLock<Component>;

/// Defines the buffer which contains the components.
pub(crate) type BufferBlockVec = BlockVec<ComponentRef, NUM_OF_COMPONETS_PER_PAGE>;

/// Defines the data structure where the components will be stored.
///
/// RwLock necessary in order to avoid problem when the vec is expanded.
///
/// The reference to the Vec must be protected due two or more thread
/// could potentially modify the same index at the same time.
pub(crate) type ComponentBuffer = Arc<RwLock<BufferBlockVec>>;

/// Defines the data structure which contains a unique component.
/// For some reason Rust does not allow me to cast from Arc<RwLock<Any>> 
/// it must be Arc<dyn Any>
pub(crate) type UniqueComponent = Arc<dyn Any + Send + Sync>;

/// Provides an aftraction to store all the components in the ECS.
pub struct ComponentsStorage {
    /// Contains all the components in the ECS.
    components: RwLock<FxHashMap<TypeId, ComponentBuffer>>,

    /// Contains all the bitmasks of the components.
    bitmasks: RwLock<FxHashMap<TypeId, u8>>,

    /// Contains all the unique components in the storage.
    unique_components: RwLock<FxHashMap<TypeId, UniqueComponent>>,
}

unsafe impl Send for ComponentsStorage {}
unsafe impl Sync for ComponentsStorage {}

/// Provides default initialization for `ComponentsStorage`.
impl Default for ComponentsStorage {
    /// Creates and returns a new `ComponentsStorage` with a default
    /// configuration.
    fn default() -> Self {
        Self {
            components: RwLock::new(FxHashMap::default()),
            bitmasks: RwLock::new(FxHashMap::default()),
            unique_components: RwLock::new(FxHashMap::default()),
        }
    }
}

impl ComponentsHandler for ComponentsStorage {
    /// Registers a component into the `Storage`.
    fn register(&self, c0: TypeId, bitmask_shift: u8) {
        {
            // Get exclusive access to the map.
            let mut c_write = self.components.write().unwrap();
            let mut bitmask_c_write = self.bitmasks.write().unwrap();
            // At this point we need create a new component buffer
            // due it does not exist.
            let new_vec = BlockVec::<ComponentRef, NUM_OF_COMPONETS_PER_PAGE>::new();
            // Insert the new buffer associated with the correct id.
            c_write.insert(c0, Arc::new(RwLock::new(new_vec)));
            // Insert the bitmask shift for the component.
            bitmask_c_write.insert(c0, bitmask_shift);
        }

        // Sync buffers, this could happen if the component is added
        // after entities.
        self.sync_buffers();
    }

    /// Registers a new unique component into the `Storage`.
    fn register_unique<C0: 'static + Send + Sync>(&self, id: TypeId, c: C0) {
        let mut u_c_writer = self.unique_components.write().unwrap();
        u_c_writer.insert(id, Arc::new(RwLock::new(Storage::new(c))));
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
        ids: (TypeId,),
        component: (A,),
    ) {
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
            let buffer: ComponentBuffer = c_buffer.clone();
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
                    were_expansions |= self.add_new_component(&entity, &buffer, component.0);
                }
            } else {
                drop(b_reader);
                were_expansions |= self.add_new_component(&entity, &buffer, component.0);
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

    /// Returns a reference to the component buffer.
    ///
    /// # Arguments
    ///
    /// `type_id` - The id of the type to be search.
    fn component_buffer(&self, type_id: &TypeId) -> Option<ComponentBuffer> {
        // Get a read lock to the components.
        let c_read = self.components.read().unwrap();
        // Check idf it can get the buffer if not just return None.
        guard!(let Some(buffer) = c_read.get(&type_id) else { return None; });
        // Returns a clone of the reference to the buffer.
        Some(buffer.clone())
    }

    /// Returns a reference to the unique component associated with the id.
    ///
    /// # Arguments
    ///
    /// `type_id`: The id of the component.
    fn unique_component(&self, type_id: &TypeId) -> Option<UniqueComponent> {
        let c_u_read = self.unique_components.read().unwrap();
        guard!(let Some(component) = c_u_read.get(&type_id) else { return None; });
        Some(component.clone())
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

impl ComponentsStorage {
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
        buffer: &ComponentBuffer,
        component: C,
    ) -> bool {
        let mut b_writer = buffer.write().unwrap();
        // If there are not items in that position we have to create
        // a new one.

        // Replace the current component with a new one.
        b_writer.set(
            RwLock::new(Some(Arc::new(RwLock::new(component)))),
            entity.id,
        )
    }
}

impl Debug for ComponentsStorage {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        let s_reader = self.components.read().unwrap();
        write!(
            formatter,
            "number of components: {:?}",
            s_reader.keys().len()
        )
    }
}
