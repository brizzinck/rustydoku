use super::{spawner::spawn_empty_figure, Figure, FigureType};
use crate::{
    components::map::{Tile, TILE_SIZE},
    resource::state_figure::StateFigure,
};
use bevy::prelude::*;

pub const SQUARE_SIZE: f32 = TILE_SIZE;

#[derive(Component, Default)]
pub struct Square {
    pub(super) parent: Option<Entity>,
}

pub(super) fn spawn(commands: &mut Commands, position: Vec2) -> Entity {
    let parent = spawn_empty_figure(commands, position, &[Vec2::new(0., 0.)]);
    let child = spawn_child(commands, parent, Vec2::new(0., 0.));
    commands.entity(parent).insert(Figure {
        squares: vec![child],
    });
    commands.entity(parent).insert(FigureType::Square);
    child
}

pub(super) fn spawn_child(commands: &mut Commands, parent: Entity, position: Vec2) -> Entity {
    let child = commands
        .spawn((
            Sprite {
                custom_size: Some(Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                ..Default::default()
            },
            Transform::from_xyz(position.x * SQUARE_SIZE, position.y * SQUARE_SIZE, 1.0),
            Square {
                parent: Some(parent),
            },
        ))
        .set_parent(parent)
        .id();

    child
}

pub(crate) fn highlight(
    mut tile_query: Query<(&Tile, &mut Sprite, &GlobalTransform, Entity)>,
    figure_query: Query<&Figure>,
    mut square_query: Query<(Entity, &GlobalTransform, &mut Square)>,
    state_figure: ResMut<StateFigure>,
) {
    for (tile, mut sprite, _, _) in tile_query.iter_mut() {
        sprite.color = tile.default_color;
    }

    if let StateFigure::Dragging(figure) = *state_figure {
        let all_tiles = tile_query
            .iter()
            .map(|(tile, _, transform, entity)| (tile, transform, entity))
            .collect::<Vec<_>>();

        let mut highlight_tiles = vec![];
        if let Ok(figure) = figure_query.get(figure) {
            for &square_entity in figure.squares.iter() {
                if let Ok((_, square_transform, _)) = square_query.get_mut(square_entity) {
                    if let Some(tile_entity) = check_for_place(square_transform, &all_tiles) {
                        highlight_tiles.push(tile_entity);
                    }
                }
            }

            if highlight_tiles.len() == figure.squares.len() {
                for tile_entity in highlight_tiles.into_iter() {
                    if let Ok((_, mut sprite, _, _)) = tile_query.get_mut(tile_entity) {
                        sprite.color = Color::srgb(0., 1., 0.);
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
