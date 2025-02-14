use crate::{
    components::{
        figures::{square::Square, Figure, FigureBounds},
        map::{Tile, MAP_SPAWN_POS, TILE_SIZE},
    },
    resource::map::Map,
    states::StateGame,
};
use bevy::prelude::*;

pub fn check_game_over(
    figures: Query<(&Transform, &FigureBounds, &Figure), Without<Square>>,
    squares: Query<&GlobalTransform, With<Square>>,
    tiles: Query<&Tile>,
    map: Res<Map>,
    state: Res<State<StateGame>>,
    mut next_state: ResMut<NextState<StateGame>>,
) {
    if StateGame::CheckGameOver == *state.get() {
        let mut game_over = true;

        'outer: for (transform, bounds, figure) in figures.iter() {
            for x in MAP_SPAWN_POS {
                for y in MAP_SPAWN_POS {
                    let new_x = x as f32 + bounds.min.x;
                    let new_y = y as f32 + bounds.min.y;
                    let new_pos = Vec2::new(new_x * TILE_SIZE, new_y * TILE_SIZE);

                    let mut valid_placement = true;
                    for square in figure.squares.iter() {
                        if let Ok(square_transform) = squares.get(*square) {
                            let candidate_pos = square_transform.translation()
                                - transform.translation
                                + new_pos.extend(0.0);

                            let grid_pos = (
                                candidate_pos.x.round() as i32,
                                candidate_pos.y.round() as i32,
                            );

                            info!("Checking candidate position: {:?}", grid_pos);

                            if correct_to_place(candidate_pos, &map, &tiles).is_none() {
                                valid_placement = false;
                                info!("Invalid placement at {:?}", grid_pos);
                                break;
                            }
                        }
                    }

                    if valid_placement {
                        game_over = false;
                        info!("Valid placement found! Game continues.");
                        break 'outer;
                    }
                }
            }
        }

        if game_over {
            info!("GAME OVER!");
            next_state.set(StateGame::GameOver);
        } else {
            next_state.set(StateGame::Idle);
        }
    }
}

pub fn correct_to_place(pos: Vec3, map: &Res<Map>, tiles: &Query<&Tile>) -> Option<(i32, i32)> {
    let position = (pos.x.round() as i32, pos.y.round() as i32);

    if let Some(tile_entity) = map.get(position) {
        if let Ok(tile) = tiles.get(*tile_entity) {
            if tile.square.is_some() {
                return None;
            }
            return Some(position);
        }
    }

    None
}
