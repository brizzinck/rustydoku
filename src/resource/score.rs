use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Score {
    pub value: i32,
}
