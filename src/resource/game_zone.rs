use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct GameZone {
    pub left_up: Vec2,
    pub right_down: Vec2,
}
