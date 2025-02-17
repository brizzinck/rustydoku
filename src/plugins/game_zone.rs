use crate::{
    constants::{
        map::{HALF_MAP_SIZE, HALF_MAP_TILE, TILE_SIZE},
        world::game_zone::DUMP_DOWN_SCALED_FACTOR,
    },
    resource::game_zone::GameZone,
};
use bevy::prelude::*;

pub struct GameZonePlugin;

impl Plugin for GameZonePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_game_zone);
    }
}

pub fn setup_game_zone(mut commands: Commands) {
    let left_up = Vec2::new(
        -HALF_MAP_SIZE + HALF_MAP_TILE,
        HALF_MAP_SIZE - HALF_MAP_TILE,
    );

    let right_down = Vec2::new(
        HALF_MAP_SIZE - HALF_MAP_TILE,
        -HALF_MAP_SIZE + HALF_MAP_TILE - TILE_SIZE * DUMP_DOWN_SCALED_FACTOR,
    );

    commands.insert_resource(GameZone {
        left_up,
        right_down,
    });
}
