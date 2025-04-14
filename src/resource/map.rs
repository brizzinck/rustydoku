use bevy::{prelude::*, utils::HashMap};

/// A resource that represents the entity map of the game world.
///
/// Internally, it stores a mapping from 2D grid positions `(i32, i32)` to [`Entity`] references,
/// allowing quick lookups of entities based on their grid location. Useful for collision checks,
/// spatial queries, and gameplay logic related to map occupancy.
#[derive(Resource, Default)]
pub struct MapComponent(pub(crate) HashMap<(i32, i32), Entity>);

impl MapComponent {
    /// Retrieves the entity located at the given grid position, if any.
    ///
    /// # Parameters
    /// - `posisition`: A 2D grid coordinate represented as a tuple `(i32, i32)`.
    ///
    /// # Returns
    /// - `Some(&Entity)` if an entity exists at the specified position.
    /// - `None` if no entity is mapped to that position.
    pub(crate) fn get(&self, posisition: (i32, i32)) -> Option<&Entity> {
        self.0.get(&posisition)
    }
}
