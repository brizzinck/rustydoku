use crate::constants::{
    map::{HALF_MAP_SIZE, HALF_MAP_TILE, TILE_SIZE},
    world::game_zone::DUMP_DOWN_SCALED_FACTOR,
};
use bevy::prelude::*;
use once_cell::sync::OnceCell;

static GAME_ZONE_LAZY: OnceCell<GameZone> = OnceCell::new();

pub struct GameZone {
    pub left_up: Vec2,
    pub right_down: Vec2,
}

impl GameZone {
    pub fn get() -> &'static GameZone {
        GAME_ZONE_LAZY.get_or_init(|| GameZone {
            left_up: Vec2::new(
                -HALF_MAP_SIZE + HALF_MAP_TILE,
                HALF_MAP_SIZE - HALF_MAP_TILE,
            ),
            right_down: Vec2::new(
                HALF_MAP_SIZE - HALF_MAP_TILE,
                -HALF_MAP_SIZE + HALF_MAP_TILE - TILE_SIZE * DUMP_DOWN_SCALED_FACTOR,
            ),
        })
    }
}
