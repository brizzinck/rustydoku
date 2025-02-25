use crate::{
    components::map::Tile,
    constants::{figure::MAX_FIGURE_USIZE_SCALED, map::*},
    resource::{map::Map, square::SquaresToDespawn},
    states::gameplay::StateGame,
};
use bevy::{prelude::*, utils::HashMap};

impl Map {
    pub(crate) fn generate_map(
        mut commands: Commands,
        mut map: ResMut<Map>,
        mut next_state: ResMut<NextState<StateGame>>,
        assets: Res<AssetServer>,
    ) {
        let parent = commands.spawn(Map::create_map()).id();

        let mut hash_titles = HashMap::with_capacity(MAP_SIZE as usize * MAP_SIZE as usize);
        for (zero_x, x) in MAP_SPAWN_POS.enumerate() {
            for (zero_y, y) in MAP_SPAWN_POS.enumerate() {
                let color = if ((zero_x / MAX_FIGURE_USIZE_SCALED)
                    + (zero_y / MAX_FIGURE_USIZE_SCALED))
                    % 2
                    == 0
                {
                    COLOR_DARK
                } else {
                    COLOR_LIGHT
                };

                let position =
                    Vec3::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, TILE_POSITION_Z);
                let tile = commands
                    .spawn(Tile::create_tile(color, position, &assets))
                    .set_parent(parent)
                    .id();

                hash_titles.insert((x as i32, y as i32), tile);
            }
        }

        map.0 = hash_titles;

        next_state.set(StateGame::Idle);
    }

    pub(crate) fn reset_tiles(
        mut square_to_despawn: ResMut<SquaresToDespawn>,
        mut tiles: Query<&mut Tile>,
    ) {
        for mut tile in tiles.iter_mut() {
            if let Some(square) = tile.square {
                square_to_despawn.add(square);
                tile.square = None;
            }
        }
    }
}
