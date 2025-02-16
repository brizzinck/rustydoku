use crate::{
    components::map::Tile,
    constants::{figure::MAX_FIGURE_USIZE, map::*},
    resource::score::Score,
    states::StateGame,
};
use bevy::prelude::*;

/// Runs all checks combination and updates the score
pub fn check_combination(
    mut commands: Commands,
    mut tiles: Query<(&mut Tile, &Transform)>,
    mut score: ResMut<Score>,
    state: Res<State<StateGame>>,
    mut next_state: ResMut<NextState<StateGame>>,
) {
    if StateGame::CheckCombo == *state.get() {
        let grid = build_grid(tiles.iter());
        let mut tiles_to_clear = Vec::new();

        check_horizontal(&grid, &mut tiles_to_clear);
        check_vertical(&grid, &mut tiles_to_clear);
        check_blocks(&grid, &mut tiles_to_clear);

        update_score(score.as_mut(), &tiles_to_clear);
        clear_tiles(&mut tiles, &mut commands, &tiles_to_clear);

        next_state.set(StateGame::CheckGameOver);
    }
}

/// Builds a 9x9 grid representation from the tile states
fn build_grid<'a, I>(tiles: I) -> [[bool; MAP_USIZE]; MAP_USIZE]
where
    I: Iterator<Item = (&'a Tile, &'a Transform)>,
{
    let mut grid = [[false; MAP_USIZE]; MAP_USIZE];

    for (tile, transform) in tiles {
        let x = ((transform.translation.x + HALF_MAP_SIZE) / TILE_SIZE).floor() as isize;
        let y = ((transform.translation.y + HALF_MAP_SIZE) / TILE_SIZE).floor() as isize;

        if x >= 0 && x < MAP_USIZE as isize && y >= 0 && y < MAP_USIZE as isize {
            grid[y as usize][x as usize] = tile.square.is_some();
        } else {
            error!("Tile position out of bounds: ({}, {})", x, y);
        }
    }

    grid
}

/// Checks for full horizontal rows and marks them for clearing
fn check_horizontal(
    grid: &[[bool; MAP_USIZE]; MAP_USIZE],
    tiles_to_clear: &mut Vec<(usize, usize)>,
) {
    for (y, data) in grid.iter().enumerate().take(MAP_USIZE) {
        if data.iter().all(|&occupied| occupied) {
            for x in 0..MAP_USIZE {
                tiles_to_clear.push((x, y));
            }
        }
    }
}

/// Checks for full vertical columns and marks them for clearing
fn check_vertical(grid: &[[bool; MAP_USIZE]; MAP_USIZE], tiles_to_clear: &mut Vec<(usize, usize)>) {
    for x in 0..MAP_USIZE {
        if (0..MAP_USIZE).all(|y| grid[y][x]) {
            for y in 0..MAP_USIZE {
                tiles_to_clear.push((x, y));
            }
        }
    }
}

/// Checks for full 3x3 blocks and marks them for clearing
fn check_blocks(grid: &[[bool; MAP_USIZE]; MAP_USIZE], tiles_to_clear: &mut Vec<(usize, usize)>) {
    let local_size = MAP_USIZE / MAX_FIGURE_USIZE;
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

/// Updates the score based on cleared tiles
fn update_score(score: &mut Score, tiles_to_clear: &[(usize, usize)]) {
    if !tiles_to_clear.is_empty() {
        let combinations = tiles_to_clear.len();
        score.current_value += combinations as i32;
        info!("Updated Score: {}", score.current_value);
    }
}

/// Clears marked tiles and sets them to free
fn clear_tiles(
    tiles: &mut Query<(&mut Tile, &Transform)>,
    commands: &mut Commands,
    tiles_to_clear: &[(usize, usize)],
) {
    for (mut tile, transform) in tiles.iter_mut() {
        if let Some(square) = tile.square {
            let tile_x = ((transform.translation.x + HALF_MAP_SIZE) / TILE_SIZE).floor() as isize;
            let tile_y = ((transform.translation.y + HALF_MAP_SIZE) / TILE_SIZE).floor() as isize;

            if tile_x >= 0
                && tile_x < MAP_USIZE as isize
                && tile_y >= 0
                && tile_y < MAP_USIZE as isize
                && tiles_to_clear.contains(&(tile_x as usize, tile_y as usize))
            {
                commands.entity(square).despawn();
                tile.square = None;
                info!("Cleared tile at ({}, {})", tile_x, tile_y);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_grid_works() {
        let mut world = World::new();

        let entity = world.spawn(()).id();
        world.spawn((
            Tile {
                square: Some(entity),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));

        world.spawn((Tile::default(), Transform::from_xyz(TILE_SIZE, 0.0, 0.0)));

        let mut tiles = world.query::<(&Tile, &Transform)>();
        let grid = build_grid(tiles.iter(&world));

        assert!(grid[4][4], "Expected tile at ({},{}) to be occupied", 4, 4);

        assert!(!grid[0][0], "Expected tile at ({},{}) to be free", 0, 0);
    }

    #[test]
    fn combo_vertical_works() {
        let mut grid = [[false; MAP_USIZE]; MAP_USIZE];

        for data in grid.iter_mut().take(MAP_USIZE) {
            data[2] = true;
        }

        let mut tiles_to_clear = Vec::new();
        check_vertical(&grid, &mut tiles_to_clear);

        assert_eq!(tiles_to_clear.len(), MAP_USIZE);
    }

    #[test]
    fn combo_horizontal_works() {
        let mut grid = [[false; MAP_USIZE]; MAP_USIZE];

        for x in 0..MAP_USIZE {
            grid[3][x] = true;
        }

        let mut tiles_to_clear = Vec::new();
        check_horizontal(&grid, &mut tiles_to_clear);

        assert_eq!(tiles_to_clear.len(), MAP_USIZE);
    }

    #[test]
    fn combo_blocks_works() {
        let mut grid = [[false; MAP_USIZE]; MAP_USIZE];

        for x in 0..3 {
            for data in grid.iter_mut().take(3) {
                data[x] = true;
            }
        }

        let mut tiles_to_clear = Vec::new();
        check_blocks(&grid, &mut tiles_to_clear);

        assert_eq!(tiles_to_clear.len(), 9);
    }

    #[test]
    fn update_score_works() {
        let mut world = World::new();
        world.insert_resource(Score {
            current_value: 0,
            max_value: 0,
        });

        let mut score = world.resource_mut::<Score>();

        let tiles_to_clear = vec![(0, 0), (1, 0), (5, 0), (6, 0), (7, 0), (8, 0)];

        update_score(&mut score, &tiles_to_clear);

        assert!(
            score.current_value > 0,
            "Expected score to increase, but got {}",
            score.current_value
        );
    }
}
