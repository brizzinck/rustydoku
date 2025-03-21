use bevy::prelude::*;

/// A resource that holds a list of square entities scheduled to be despawned.
///
/// This resource is used to coordinate and animate square despawning actions,
/// allowing systems to process and animate the despawn over time, rather than immediately.
///
/// Typically used in puzzle or grid-based games where multiple squares disappear
/// at once (e.g. after clearing a line or matching blocks).
#[derive(Resource, Default)]
pub struct SquaresToDespawnResource {
    /// A list of entities representing squares that should be despawned.
    pub(crate) squares: Vec<Entity>,
}

impl SquaresToDespawnResource {
    /// Adds a square entity to the despawn queue.
    ///
    /// This does not immediately despawn the square, but marks it for future
    /// processing by an animation or cleanup system.
    ///
    /// # Parameters
    /// - `square`: The [`Entity`] representing the square to be marked for despawn.
    pub(crate) fn add(&mut self, square: Entity) {
        self.squares.push(square);
    }
}
