use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Score {
    pub(crate) value: i32,
}
