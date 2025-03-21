use crate::{
    components::world::map::TileComponent,
    constants::{figure::MAX_FIGURE_USIZE_SCALED, map::*},
    resource::{map::MapComponent, square::SquaresToDespawnResource},
    states::gameplay::GameState,
};
use assets::{TILE_IMAGE_FIRST_DEACTIVATED_PATH, TILE_IMAGE_SECOND_DEACTIVATED_PATH};
use bevy::{prelude::*, utils::HashMap};

impl MapComponent {
    pub(crate) fn generate_map(
        mut commands: Commands,
        mut map: ResMut<MapComponent>,
        mut next_state: ResMut<NextState<GameState>>,
        assets: Res<AssetServer>,
    ) {
        let parent = commands.spawn(MapComponent::create_map()).id();

        let mut hash_titles = HashMap::with_capacity(MAP_SIZE as usize * MAP_SIZE as usize);
        for (zero_x, x) in MAP_SPAWN_POSITIOM.enumerate() {
            for (zero_y, y) in MAP_SPAWN_POSITIOM.enumerate() {
                let image = if ((zero_x / MAX_FIGURE_USIZE_SCALED)
                    + (zero_y / MAX_FIGURE_USIZE_SCALED))
                    % 2
                    == 0
                {
                    assets.load(TILE_IMAGE_SECOND_DEACTIVATED_PATH)
                } else {
                    assets.load(TILE_IMAGE_FIRST_DEACTIVATED_PATH)
                };

                let position =
                    Vec3::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, TILE_Z_POSITION);
                let tile = commands
                    .spawn(TileComponent::create_tile(image, position))
                    .set_parent(parent)
                    .id();

                hash_titles.insert((x as i32, y as i32), tile);
            }
        }

        map.0 = hash_titles;

        next_state.set(GameState::Idle);
        trace!("Next state StateGame::Idle");
    }

    pub(crate) fn reset_tiles(
        mut square_to_despawn: ResMut<SquaresToDespawnResource>,
        mut tiles: Query<&mut TileComponent>,
    ) {
        trace!("Reset tiles");

        for mut tile in tiles.iter_mut() {
            if let Some(square) = tile.square {
                square_to_despawn.add(square);
                tile.square = None;
            }
        }
    }
}
