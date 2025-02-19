use bevy::{prelude::*, utils::HashMap};

#[derive(Resource, Default)]
pub struct Map(pub(crate) HashMap<(i32, i32), Entity>);

impl Map {
    pub(crate) fn get(&self, pos: (i32, i32)) -> Option<&Entity> {
        self.0.get(&pos)
    }
}
