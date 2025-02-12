use crate::{
    components::map::{Tile, MAP_SIZE, TILE_SIZE},
    resource::score::Score,
};
use bevy::prelude::*;

const GRID_SIZE: usize = MAP_SIZE as usize;

/// Runs all checks combination and updates the score
pub fn check_combination(
    mut commands: Commands,
    mut tiles: Query<(&mut Tile, &Transform)>,
    mut score: ResMut<Score>,
) {
    let grid = build_grid(tiles.iter());
    let mut tiles_to_clear = Vec::new();

    check_horizontal(&grid, &mut tiles_to_clear);
    check_vertical(&grid, &mut tiles_to_clear);
    check_blocks(&grid, &mut tiles_to_clear);

    update_score(score.as_mut(), &tiles_to_clear);
    clear_tiles(&mut tiles, &mut commands, &tiles_to_clear);
}

/// Builds a 9x9 grid representation from the tile states
fn build_grid<'a, I>(tiles: I) -> [[bool; GRID_SIZE]; GRID_SIZE]
where
    I: Iterator<Item = (&'a Tile, &'a Transform)>,
{
    let mut grid = [[false; GRID_SIZE]; GRID_SIZE];

    let half_map_size = (MAP_SIZE as f32 * TILE_SIZE) / 2.0;

    for (tile, transform) in tiles {
        let x = ((transform.translation.x + half_map_size) / TILE_SIZE).floor() as isize;
        let y = ((transform.translation.y + half_map_size) / TILE_SIZE).floor() as isize;

        if x >= 0 && x < GRID_SIZE as isize && y >= 0 && y < GRID_SIZE as isize {
            grid[y as usize][x as usize] = tile.square.is_some();
        } else {
            error!("Tile position out of bounds: ({}, {})", x, y);
        }
    }

    grid
}

/// Checks for full horizontal rows and marks them for clearing
fn check_horizontal(
    grid: &[[bool; GRID_SIZE]; GRID_SIZE],
    tiles_to_clear: &mut Vec<(usize, usize)>,
) {
    for (y, data) in grid.iter().enumerate().take(GRID_SIZE) {
        if data.iter().all(|&occupied| occupied) {
            for x in 0..GRID_SIZE {
                tiles_to_clear.push((x, y));
            }
        }
    }
}

/// Checks for full vertical columns and marks them for clearing
fn check_vertical(grid: &[[bool; GRID_SIZE]; GRID_SIZE], tiles_to_clear: &mut Vec<(usize, usize)>) {
    for x in 0..GRID_SIZE {
        if (0..GRID_SIZE).all(|y| grid[y][x]) {
            for y in 0..GRID_SIZE {
                tiles_to_clear.push((x, y));
            }
        }
    }
}

/// Checks for full 3x3 blocks and marks them for clearing
fn check_blocks(grid: &[[bool; GRID_SIZE]; GRID_SIZE], tiles_to_clear: &mut Vec<(usize, usize)>) {
    let local_size = GRID_SIZE / 3;
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
        score.value += combinations as i32;
        info!("Updated Score: {}", score.value);
    }
}

/// Clears marked tiles and sets them to free
fn clear_tiles(
    tiles: &mut Query<(&mut Tile, &Transform)>,
    commands: &mut Commands,
    tiles_to_clear: &[(usize, usize)],
) {
    let half_map_size = (MAP_SIZE as f32 * TILE_SIZE) / 2.0;

    for (mut tile, transform) in tiles.iter_mut() {
        if let Some(square) = tile.square {
            let tile_x = ((transform.translation.x + half_map_size) / TILE_SIZE).floor() as isize;
            let tile_y = ((transform.translation.y + half_map_size) / TILE_SIZE).floor() as isize;

            if tile_x >= 0
                && tile_x < GRID_SIZE as isize
                && tile_y >= 0
                && tile_y < GRID_SIZE as isize
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
        let mut grid = [[false; GRID_SIZE]; GRID_SIZE];

        for data in grid.iter_mut().take(GRID_SIZE) {
            data[2] = true;
        }

        let mut tiles_to_clear = Vec::new();
        check_vertical(&grid, &mut tiles_to_clear);

        assert_eq!(tiles_to_clear.len(), GRID_SIZE);
    }

    #[test]
    fn combo_horizontal_works() {
        let mut grid = [[false; GRID_SIZE]; GRID_SIZE];

        for x in 0..GRID_SIZE {
            grid[3][x] = true;
        }

        let mut tiles_to_clear = Vec::new();
        check_horizontal(&grid, &mut tiles_to_clear);

        assert_eq!(tiles_to_clear.len(), GRID_SIZE);
    }

    #[test]
    fn combo_blocks_works() {
        let mut grid = [[false; GRID_SIZE]; GRID_SIZE];

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
        world.insert_resource(Score { value: 0 });

        let mut score = world.resource_mut::<Score>();

        let tiles_to_clear = vec![(0, 0), (1, 0), (5, 0), (6, 0), (7, 0), (8, 0)];

        update_score(&mut score, &tiles_to_clear);

        assert!(
            score.value > 0,
            "Expected score to increase, but got {}",
            score.value
        );
    }
}
