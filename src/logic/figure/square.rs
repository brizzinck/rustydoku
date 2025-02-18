use crate::{
    components::{
        figures::{
            square::{create_child, Square},
            Figure,
        },
        map::Tile,
    },
    constants::{figure::FIGURE_DRAGGING_SCALE, map::TILE_SIZE},
    states::gameplay::StateGame,
};
use bevy::prelude::*;

pub fn spawn_as_child(
    commands: &mut Commands,
    parent: Entity,
    position: Vec2,
    rotation: Quat,
    assets: &Res<AssetServer>,
) -> Entity {
    let child = commands
        .spawn(create_child(parent, position, rotation, assets))
        .set_parent(parent)
        .id();
    child
}

pub(crate) fn highlight(
    mut tile_query: Query<(&Tile, &mut Sprite, &GlobalTransform, Entity)>,
    figure_query: Query<(&Figure, &Transform)>,
    mut square_query: Query<(Entity, &GlobalTransform, &mut Square)>,
    current_state: Res<State<StateGame>>,
) {
    for (tile, mut sprite, _, _) in tile_query.iter_mut() {
        sprite.color = tile.default_color;
    }

    if let StateGame::Dragging(figure) = current_state.get() {
        let mut highlight_tiles = vec![];
        if let Ok((figure, transform)) = figure_query.get(*figure) {
            if transform.scale.x < FIGURE_DRAGGING_SCALE {
                return;
            }

            let tiles = tile_query
                .iter()
                .map(|(tile, _, transform, entity)| (tile, transform, entity))
                .collect::<Vec<_>>();

            for &square_entity in figure.squares_entity.iter() {
                if let Ok((_, square_transform, _)) = square_query.get_mut(square_entity) {
                    if let Some(tile_entity) = check_for_place(square_transform, &tiles) {
                        highlight_tiles.push(tile_entity);
                    }
                }
            }

            if highlight_tiles.len() == figure.squares_entity.len() {
                for tile_entity in highlight_tiles.into_iter() {
                    if let Ok((_, mut sprite, _, _)) = tile_query.get_mut(tile_entity) {
                        sprite.color = Color::srgb(100., 100., 100.); // TODO: Change to image
                                                                      // maybe
                    }
                }
            }
        }
    }
}

pub fn check_for_place(
    transofrm: &GlobalTransform,
    tile_query: &Vec<(&Tile, &GlobalTransform, Entity)>,
) -> Option<Entity> {
    let square_pos = transofrm.translation().truncate();
    let square_grid_x = (square_pos.x / TILE_SIZE).round() as i32;
    let square_grid_y = (square_pos.y / TILE_SIZE).round() as i32;

    for (tile, tile_transform, tile_entity) in tile_query.iter() {
        let tile_pos = tile_transform.translation().truncate();
        let tile_grid_x = (tile_pos.x / TILE_SIZE).round() as i32;
        let tile_grid_y = (tile_pos.y / TILE_SIZE).round() as i32;

        if (square_grid_x == tile_grid_x && square_grid_y == tile_grid_y) && tile.square.is_none() {
            return Some(*tile_entity);
        }
    }

    None
}
