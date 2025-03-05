use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct SquaresToDespawn {
    pub(crate) squares: Vec<Entity>,
}

impl SquaresToDespawn {
    pub(crate) fn add(&mut self, square: Entity) {
        self.squares.push(square);
    }
}
