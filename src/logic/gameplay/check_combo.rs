use crate::{
    components::world::map::TileComponent,
    constants::{figure::MAX_FIGURE_USIZE_SCALED, map::*},
    events::gameplay::ComboEvent,
    resource::{score::ScoreResource, square::SquaresToDespawnResource},
    states::gameplay::GameState,
    world::gameplay::Gameplay,
};
use bevy::prelude::*;

impl Gameplay {
    /// Checks for completed combinations (rows(9), columns(9), 3x3 blocks) and handles scoring and clearing.
    ///
    /// This method:
    /// - Builds a 2D grid representing the current tile occupancy.
    /// - Checks for any full rows, columns, or 3x3 blocks.
    /// - Marks all filled tile positions for clearing.
    /// - Updates the player's score.
    /// - Sends a [`ComboEvent`] if any lines/blocks were cleared.
    /// - Despawns affected squares.
    /// - Advances the game state to [`GameState::CheckGameOver`].
    pub(crate) fn check_combination(
        mut tiles: Query<(&mut TileComponent, &Transform)>,
        mut score: ResMut<ScoreResource>,
        mut next_game_state: ResMut<NextState<GameState>>,
        mut squares_to_despawn: ResMut<SquaresToDespawnResource>,
        mut evemt_writer: EventWriter<ComboEvent>,
    ) {
        let grid = Self::build_grid(tiles.iter());
        let mut tiles_to_clear = Vec::new();

        Self::check_horizontal(&grid, &mut tiles_to_clear);
        Self::check_vertical(&grid, &mut tiles_to_clear);
        Self::check_blocks(&grid, &mut tiles_to_clear);

        Self::update_score(score.as_mut(), &tiles_to_clear);

        if !tiles_to_clear.is_empty() {
            evemt_writer.send(ComboEvent);
        }

        Self::clear_tiles(&mut tiles, &tiles_to_clear, &mut squares_to_despawn);

        next_game_state.set(GameState::CheckGameOver);
    }

    /// Builds a 2D boolean grid (9x9) from the current tile positions and occupancy.
    ///
    /// Each grid cell represents whether a tile contains a square (`true`) or is empty (`false`).
    ///
    /// # Parameters
    /// - `tiles`: Iterator over tile components and their transforms.
    ///
    /// # Returns
    /// A 9x9 grid of booleans.
    fn build_grid<'a, I>(tiles: I) -> [[bool; MAP_SIZE_USIZE]; MAP_SIZE_USIZE]
    where
        I: Iterator<Item = (&'a TileComponent, &'a Transform)>,
    {
        let mut grid = [[false; MAP_SIZE_USIZE]; MAP_SIZE_USIZE];

        for (tile, transform) in tiles {
            let x = ((transform.translation.x + HALF_MAP_SIZE) / TILE_SIZE).floor() as isize;
            let y = ((transform.translation.y + HALF_MAP_SIZE) / TILE_SIZE).floor() as isize;

            if x >= 0 && x < MAP_SIZE_USIZE as isize && y >= 0 && y < MAP_SIZE_USIZE as isize {
                grid[y as usize][x as usize] = tile.square.is_some();
            } else {
                error!("Tile position out of bounds: ({}, {})", x, y);
            }
        }

        grid
    }

    /// Checks for full horizontal rows and marks them for clearing
    fn check_horizontal(
        grid: &[[bool; MAP_SIZE_USIZE]; MAP_SIZE_USIZE],
        tiles_to_clear: &mut Vec<(usize, usize)>,
    ) {
        for (y, data) in grid.iter().enumerate().take(MAP_SIZE_USIZE) {
            if data.iter().all(|&occupied| occupied) {
                for x in 0..MAP_SIZE_USIZE {
                    tiles_to_clear.push((x, y));
                }
            }
        }
    }

    /// Checks for full vertical columns and marks them for clearing
    fn check_vertical(
        grid: &[[bool; MAP_SIZE_USIZE]; MAP_SIZE_USIZE],
        tiles_to_clear: &mut Vec<(usize, usize)>,
    ) {
        for x in 0..MAP_SIZE_USIZE {
            if (0..MAP_SIZE_USIZE).all(|y| grid[y][x]) {
                for y in 0..MAP_SIZE_USIZE {
                    tiles_to_clear.push((x, y));
                }
            }
        }
    }

    /// Checks for full 3x3 blocks and marks them for clearing
    fn check_blocks(
        grid: &[[bool; MAP_SIZE_USIZE]; MAP_SIZE_USIZE],
        tiles_to_clear: &mut Vec<(usize, usize)>,
    ) {
        let local_size = MAP_SIZE_USIZE / MAX_FIGURE_USIZE_SCALED;
        for i in 0..local_size {
            for j in 0..local_size {
                let mut is_full = true;

                for x in 0..local_size {
                    for y in 0..local_size {
                        if !grid[j * local_size + y][i * local_size + x] {
                            is_full = false;
                            break;
                        }
                    }
                }

                if is_full {
                    for x in 0..local_size {
                        for y in 0..local_size {
                            tiles_to_clear.push((i * local_size + x, j * local_size + y));
                        }
                    }
                }
            }
        }
    }

    /// Adds score based on the number of cleared tiles.
    ///
    /// # Parameters
    /// - `score`: Mutable reference to the score resource.
    /// - `tiles_to_clear`: List of tiles that will be cleared.
    fn update_score(score: &mut ScoreResource, tiles_to_clear: &[(usize, usize)]) {
        if !tiles_to_clear.is_empty() {
            let combinations = tiles_to_clear.len();
            score.add_score(combinations as i32);
            info!("Updated Score: {}", score.get_current_score());
        }
    }

    /// Clears the marked tiles and queues their squares for despawning.
    ///
    /// # Parameters
    /// - `tiles`: Query of tile components and transforms.
    /// - `tiles_to_clear`: Positions of tiles to clear.
    /// - `squares_to_despawn`: Resource holding square entities to be removed.
    fn clear_tiles(
        tiles: &mut Query<(&mut TileComponent, &Transform)>,
        tiles_to_clear: &[(usize, usize)],
        squares_to_despawn: &mut ResMut<SquaresToDespawnResource>,
    ) {
        for (mut tile, transform) in tiles.iter_mut() {
            if let Some(square) = tile.square {
                let tile_x =
                    ((transform.translation.x + HALF_MAP_SIZE) / TILE_SIZE).floor() as isize;
                let tile_y =
                    ((transform.translation.y + HALF_MAP_SIZE) / TILE_SIZE).floor() as isize;

                if tile_x >= 0
                    && tile_x < MAP_SIZE_USIZE as isize
                    && tile_y >= 0
                    && tile_y < MAP_SIZE_USIZE as isize
                    && tiles_to_clear.contains(&(tile_x as usize, tile_y as usize))
                {
                    squares_to_despawn.add(square);
                    tile.square = None;

                    info!("Cleared tile at ({}, {})", tile_x, tile_y);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies grid generation from tile transform positions.
    #[test]
    fn build_grid_works() {
        let mut world = World::new();

        let entity = world.spawn(()).id();
        world.spawn((
            TileComponent {
                square: Some(entity),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));

        world.spawn((
            TileComponent::default(),
            Transform::from_xyz(TILE_SIZE, 0.0, 0.0),
        ));

        let mut tiles = world.query::<(&TileComponent, &Transform)>();
        let grid = Gameplay::build_grid(tiles.iter(&world));

        assert!(grid[4][4], "Expected tile at ({},{}) to be occupied", 4, 4);

        assert!(!grid[0][0], "Expected tile at ({},{}) to be free", 0, 0);
    }

    /// Verifies detection of full vertical columns.
    #[test]
    fn combo_vertical_works() {
        let mut grid = [[false; MAP_SIZE_USIZE]; MAP_SIZE_USIZE];

        for data in grid.iter_mut().take(MAP_SIZE_USIZE) {
            data[2] = true;
        }

        let mut tiles_to_clear = Vec::new();
        Gameplay::check_vertical(&grid, &mut tiles_to_clear);

        assert_eq!(tiles_to_clear.len(), MAP_SIZE_USIZE);
    }

    /// Verifies detection of full horizontal rows.
    #[test]
    fn combo_horizontal_works() {
        let mut grid = [[false; MAP_SIZE_USIZE]; MAP_SIZE_USIZE];

        for x in 0..MAP_SIZE_USIZE {
            grid[3][x] = true;
        }

        let mut tiles_to_clear = Vec::new();
        Gameplay::check_horizontal(&grid, &mut tiles_to_clear);

        assert_eq!(tiles_to_clear.len(), MAP_SIZE_USIZE);
    }

    /// Verifies detection of full 3x3 blocks.
    #[test]
    fn combo_blocks_works() {
        let mut grid = [[false; MAP_SIZE_USIZE]; MAP_SIZE_USIZE];

        for x in 0..3 {
            for data in grid.iter_mut().take(3) {
                data[x] = true;
            }
        }

        let mut tiles_to_clear = Vec::new();
        Gameplay::check_blocks(&grid, &mut tiles_to_clear);

        assert_eq!(tiles_to_clear.len(), 9);
    }

    /// Verifies that score increases based on the number of cleared tiles.
    #[test]
    fn update_score_works() {
        let mut world = World::new();
        world.insert_resource(ScoreResource::default());

        let mut score = world.resource_mut::<ScoreResource>();

        let tiles_to_clear = vec![(0, 0), (1, 0), (5, 0), (6, 0), (7, 0), (8, 0)];

        Gameplay::update_score(&mut score, &tiles_to_clear);

        assert!(
            score.get_current_score() > 0,
            "Expected score to increase, but got {}",
            score.get_current_score()
        );
    }
}
