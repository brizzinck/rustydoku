use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct SquaresToDespawnResource {
    pub(crate) squares: Vec<Entity>,
}

impl SquaresToDespawnResource {
    pub(crate) fn add(&mut self, square: Entity) {
        self.squares.push(square);
    }
}
