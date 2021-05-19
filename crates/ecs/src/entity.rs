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

    /// An aftraction used to search for all the entities which 
    fn query_by_bitmask(&self, bitmasks: BitmaskType) -> Vec<Entity>;
}

/// Represents a storage which holds entities.
pub struct EntitiesStorage<const N: usize> {
    /// Contains a list of all the masks.
    bit_masks: RwLock<BlockVec<BitmaskType, N>>
}

unsafe impl<const N: usize> Send for EntitiesStorage<N> {}
unsafe impl<const N: usize> Sync for EntitiesStorage<N> {}

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
    /// 
    /// TODO(Angel): Try to only lock the item itself and not the 
    /// entire array.
    fn reset_bitmask(&self, entity: &Entity) {
        // Get a write lock.
        let mut cm_writer = self.bit_masks.write().unwrap();
        // Clear the bitmask.
        cm_writer.set(0, entity.id);
    }

    /// Returns a list of entities which cumpliments with the
    /// bitmask requirement.
    /// 
    /// # Arguments
    /// 
    /// `bitmasks` - The bitmask filter.
    /// 
    /// TODO(Angel): Huge optimization here, it is not needed to
    /// iterate over all the entire vec just till the last element
    /// but that required `BlockVec` modifications.
    fn query_by_bitmask(&self, bitmasks: BitmaskType) -> Vec<Entity> {
        // A list of filtered entities.
        let mut f_entities: Vec<Entity> = Vec::new();
        // The read access to the masks. 
        let r_bitmasks = self.bit_masks.read().unwrap();
        // Get the length of the vector.
        let actual_len = r_bitmasks.actual_len();
        
        // As bitmask is setted to 0 when it is deleted the filter
        // will ignore them.
        for i in 0..actual_len {
            // The entity bitmask.
            if let Some(e_bitmask) = r_bitmasks.get(i) {
                // Apply a logical "and" over the masks, if the result
                // is equal to the mask provided then the entity 
                // contains all the needed components.
                if e_bitmask & bitmasks == bitmasks {
                    f_entities.push(Entity::new(i));
                }
            }
        } 
        
        f_entities
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
