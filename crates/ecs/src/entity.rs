use crate::bundle::ComponentBundler;

/// Provides an aftraction to handle entities.
pub trait EntityHandler {
    /// Defines an interface to add new entities.
    fn add_entity<B: ComponentBundler>(
        &self,
        components: B) -> Entity;

    /// Defines an interface to delete entities.
    fn remove_entity(&mut self, entity: Entity);
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
    pub(crate) fn new(id: EntityId) -> Self {
        Self {
            id
        }
    }
}