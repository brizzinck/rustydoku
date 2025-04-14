use crate::{
    components::world::map::TileComponent,
    constants::{figure::MAX_FIGURE_USIZE_SCALED, map::*},
    resource::{map::MapComponent, square::SquaresToDespawnResource},
    states::gameplay::GameState,
};
use assets::{TILE_IMAGE_FIRST_DEACTIVATED_PATH, TILE_IMAGE_SECOND_DEACTIVATED_PATH};
use bevy::{prelude::*, utils::HashMap};

impl MapComponent {
    /// Generates the map grid with tile entities and transitions the game state to [`GameState::Idle`].
    ///
    /// This function does the following:
    /// - Spawns a parent entity for the map grid.
    /// - Iterates over [`MAP_SPAWN_POSITIOM`] to create a 2D grid of tiles.
    /// - Applies alternating deactivated tile images.
    /// - Calculates world positions for each tile.
    /// - Stores tile entities in the internal `HashMap<(i32, i32), Entity>`.
    /// - Sets the next game state to `Idle` to signal that the board is ready.
    ///
    /// # Parameters
    /// - `commands`: Bevy's command used for spawning and modifying entities.
    /// - `map`: Mutable reference to the [`MapComponent`] resource, where tile mappings are stored.
    /// - `next_state`: Used to queue the next game state (`Idle` after map generation).
    /// - `assets`: The [`AssetServer`] for loading tile textures.
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

    /// Resets all tiles by removing associated squares and marking them for despawn.
    ///
    /// This method is used when restarting the game or regenerating the board.
    /// It does not immediately despawn the squares but adds them to the [`SquaresToDespawnResource`],
    /// allowing for a delayed or animated despawn process.
    ///
    /// # Parameters
    /// - `square_to_despawn`: A mutable reference to the resource tracking squares that should be despawned.
    /// - `tiles`: Query providing mutable access to all tile components on the map.
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
