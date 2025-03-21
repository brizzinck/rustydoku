use bevy::{prelude::*, utils::HashMap};

#[derive(Resource, Default)]
pub struct MapComponent(pub(crate) HashMap<(i32, i32), Entity>);

impl MapComponent {
    pub(crate) fn get(&self, pos: (i32, i32)) -> Option<&Entity> {
        self.0.get(&pos)
    }
}
