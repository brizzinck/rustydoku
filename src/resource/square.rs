use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct SquaresToDespawn {
    pub squares: Vec<Entity>,
}

impl SquaresToDespawn {
    pub fn add(&mut self, square: Entity) {
        self.squares.push(square);
    }

    pub fn remove(&mut self, id: usize) {
        self.squares.remove(id);
    }
}
