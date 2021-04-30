use std::sync::RwLock;

use utils::BlockVec;

use crate::{
    bundle::ComponentBundler,
    consts::BitmaskType
};

/// Pro and aftraction to handle the entitines in the storages.
pub trait EntitiesHandler {
    /// Provides an aftraction to regsiter a bigmask for an entity.
    fn register_bitmask(&self, entity: &Entity, bit_mask: &BitmaskType);

    /// Returns the bit mask associated with the provided entity.
    fn get_bitmask(&self, entity: &Entity) -> BitmaskType;

    /// An aftraction to reset the mask of the entity.
    fn reset_bitmask(&self, entity: &Entity);
}

/// Represents a storage which holds entities.
pub struct EntitiesStorage<const N: usize> {
    /// Contains a list of all the masks.
    bit_masks: RwLock<BlockVec<BitmaskType, N>>
}

impl<const N: usize> Default for EntitiesStorage<N> {
    /// Creates and returns a new `EntitiesStorage`.
    fn default() -> Self {
        Self {
            bit_masks: RwLock::new(BlockVec::new())
        }
    }
}

impl<const N: usize> EntitiesHandler for EntitiesStorage<N> {
    /// Registers the bit mask associated with the entity.
    ///
    /// # Arguments
    ///
    /// `entity` - The entity to be registered.
    /// `bit_mask` - The associated bitmask.
    fn register_bitmask(&self, entity: &Entity, bit_mask: &BitmaskType) {
        // Get a write lock of the bit masks.
        let mut cm_writer = self.bit_masks.write().unwrap();
        // Add or override the mask.
        cm_writer.set(bit_mask.clone(), entity.id);
    }

    /// Returns the bit mask for the given entity.
    ///
    /// # Arguments
    ///
    /// `entity` - The entity used to return the bitmask.
    fn get_bitmask(&self, entity: &Entity) -> BitmaskType {
        // Get a read lock of the bit masks.
        let cm_reader = self.bit_masks.read().unwrap();
        // Return the bit mask.
        let index: usize = entity.id as usize; 
        guard!(let Some(bit_mask) = cm_reader.get(index) else {
            panic!(
                "There is not bit mask associated with the entity {}",
                entity.id
            );
        });

        bit_mask.clone()
    }

    /// Rests the bitmask of the given entity.
    ///
    /// # Arguments
    ///
    /// `entity` - The entity used to find the mask to reset.
    fn reset_bitmask(&self, entity: &Entity) {
        // Get a write lock.
        let mut cm_writer = self.bit_masks.write().unwrap();
        // Clear the bitmask.
        cm_writer.set(0, entity.id);
    }
}

/// Provides an aftraction to handle entities.
pub trait EntityHandler {
    /// Defines an interface to add new entities.
    fn add_entity<B: ComponentBundler>(
        &self,
        components: B) -> Entity;
 
    /// Defines an interface to delete entities.
    fn remove_entity(&self, entity: Entity);
}

/// Defines the size of the entities id.
pub(crate) type EntityId = usize;

/// Represents an Entity in the ECS.
#[derive(Copy, Clone)]
pub struct Entity {
    /// Conatins the unique id of the entity.
    pub(crate) id: EntityId
}

impl Entity {
    /// Creates and returns a new entity.
    /// 
    /// # Arguments
    /// 
    /// `id` - The id for the entity.
    pub fn new(id: EntityId) -> Self {
        Self {
            id
        }
    }
}
