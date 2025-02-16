use bevy::prelude::*;

use crate::{
    constants::map::{MAP_FSIZE, TILE_SIZE},
    resource::game_zone::GameZone,
};

pub struct GameZonePlugin;

impl Plugin for GameZonePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_game_zone);
    }
}

pub fn setup_game_zone(mut commands: Commands) {
    let half_map_size = (MAP_FSIZE * TILE_SIZE) / 2.0;
    let left_up = Vec2::new(
        -half_map_size + TILE_SIZE / 2.0,
        half_map_size - TILE_SIZE / 2.0,
    );

    let right_down = Vec2::new(
        half_map_size - TILE_SIZE / 2.0,
        -half_map_size + TILE_SIZE / 2.0 - TILE_SIZE * 5.,
    );

    commands.insert_resource(GameZone {
        left_up,
        right_down,
    });
}
