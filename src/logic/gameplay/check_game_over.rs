use crate::{
    components::{
        figure::{Figure, FigureBounds},
        world::map::Tile,
    },
    constants::map::{MAP_SPAWN_POSITIOM, TILE_SIZE},
    events::figure::{FigureCanPlaced, FigureCantPlaced},
    resource::map::Map,
    states::gameplay::StateGame,
};
use bevy::prelude::*;

pub(crate) fn check_game_over(
    figures: Query<(&Transform, &FigureBounds, &Figure)>,
    map: Res<Map>,
    tiles: Query<&Tile>,
    mut next_state: ResMut<NextState<StateGame>>,
    mut cant_place_writer: EventWriter<FigureCantPlaced>,
    mut can_place_writer: EventWriter<FigureCanPlaced>,
) {
    let mut game_over = true;

    for (transform, bounds, figure) in figures.iter() {
        info!(
            "Checking figure {:?} with bounds.min={:?}, squares_offset={:?}",
            transform.translation, bounds.min, figure.squares_position
        );

        let mut figure_cant_placed = true;
        'outer: for grid_x in MAP_SPAWN_POSITIOM {
            for grid_y in MAP_SPAWN_POSITIOM {
                let grid = Vec2::new(grid_x as f32, grid_y as f32);

                if can_place_figure_at_grid(
                    grid,
                    bounds.min,
                    &figure.squares_position,
                    transform,
                    &map,
                    &tiles,
                ) {
                    info!(
                        "Found a valid placement for figure at grid=({}, {}) => Game continues!",
                        grid_x, grid_y
                    );
                    figure_cant_placed = false;
                    game_over = false;
                    break 'outer;
                }
            }
        }

        if figure_cant_placed {
            cant_place_writer.send(FigureCantPlaced(figure.placeholder));
        } else {
            can_place_writer.send(FigureCanPlaced(figure.placeholder));
        }
    }

    if game_over {
        info!("No valid placement found for any figure => GAME OVER!");
        next_state.set(StateGame::GameOver);
    } else {
        info!("At least one figure can be placed => continuing game!");
        next_state.set(StateGame::Idle);
    }
}

fn can_place_figure_at_grid(
    grid: Vec2,
    bounds_min: Vec2,
    offsets: &[Vec2],
    figure_transform: &Transform,
    map: &Map,
    tiles: &Query<&Tile>,
) -> bool {
    let placement_translation = grid * TILE_SIZE - bounds_min * TILE_SIZE;

    info!(
        "Trying grid=({:.0}, {:.0}); computed placement_translation={:?}",
        grid.x, grid.y, placement_translation
    );

    for offset in offsets {
        let rotated_offset = figure_transform.rotation * offset.extend(0.0);
        let candidate_pos = placement_translation + rotated_offset.truncate() * TILE_SIZE;

        info!(
            " -> Checking square offset={:?}; rotated_offset={:?}; candidate_pos={:?}",
            offset, rotated_offset, candidate_pos
        );

        if correct_to_place(candidate_pos.extend(0.0), map, tiles).is_none() {
            info!("Cannot place at {:?}", candidate_pos);
            return false;
        }
        info!("Square can be placed at {:?}", candidate_pos);
    }
    info!("All squares OK at grid=({:.0}, {:.0})", grid.x, grid.y);
    true
}

fn correct_to_place(pos: Vec3, map: &Map, tiles: &Query<&Tile>) -> Option<(i32, i32)> {
    let tile_coords = (
        (pos.x / TILE_SIZE).round() as i32,
        (pos.y / TILE_SIZE).round() as i32,
    );

    if let Some(tile_entity) = map.get(tile_coords) {
        if let Ok(tile) = tiles.get(*tile_entity) {
            if tile.square.is_none() {
                return Some(tile_coords);
            } else {
                info!("Tile at {:?} is occupied by {:?}", tile_coords, tile.square);
            }
        } else {
            info!("Could not query tile at {:?}", tile_coords);
        }
    } else {
        info!("No tile found in the map for {:?}", tile_coords);
    }
    None
}
