use super::{spawner::spawn_empty_figure, Figure};
use crate::components::map::{Tile, TILE_SIZE};
use bevy::prelude::*;

pub const SQUARE_SIZE: f32 = TILE_SIZE;

#[derive(Default, PartialEq, Debug)]
pub enum SquareState {
    #[default]
    Idle,
    Dragging,
    Placing(Vec2),
    MustBePlaced(Vec2),
    Placed(Vec2),
}

#[derive(Component, Default)]
pub struct Square {
    pub(super) state: SquareState,
}

pub(super) fn spawn(commands: &mut Commands, position: Vec2) {
    let parent = spawn_empty_figure(commands, position);
    spawn_child(commands, parent, Vec2::new(-0., 0.));
}

pub(super) fn spawn_child(commands: &mut Commands, parent: Entity, position: Vec2) {
    commands
        .spawn((
            Sprite {
                custom_size: Some(Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                ..Default::default()
            },
            Transform::from_xyz(position.x * SQUARE_SIZE, position.y * SQUARE_SIZE, 1.0),
            Square::default(),
        ))
        .set_parent(parent);
}

pub(crate) fn highlight_tile(
    mut tile_query: Query<(&Tile, &mut Sprite, &GlobalTransform, Entity)>,
    figure_query: Query<(&mut Children, &Figure)>,
    mut square_query: Query<(Entity, &GlobalTransform, &mut Square)>,
) {
    let mut squares_on_tile = Vec::new();
    let mut count = 0;

    for (children, figure) in figure_query.iter() {
        if !figure.is_dragging {
            continue;
        }

        count = children.len();

        for &child in children.iter() {
            if let Ok((square_entity, square_transform, mut square)) = square_query.get_mut(child) {
                square.state = SquareState::Dragging;

                let square_pos = square_transform.translation().truncate();
                let square_grid_x = (square_pos.x / TILE_SIZE).round() as i32;
                let square_grid_y = (square_pos.y / TILE_SIZE).round() as i32;

                for (tile, mut sprite, tile_transform, tile_entity) in tile_query.iter_mut() {
                    sprite.color = tile.default_color;

                    let tile_pos = tile_transform.translation().truncate();
                    let tile_grid_x = (tile_pos.x / TILE_SIZE).round() as i32;
                    let tile_grid_y = (tile_pos.y / TILE_SIZE).round() as i32;

                    if square_grid_x == tile_grid_x && square_grid_y == tile_grid_y && tile.is_free
                    {
                        squares_on_tile.push((square_entity, tile_pos, tile_entity));
                    }
                }
            }
        }

        break;
    }

    if !squares_on_tile.is_empty() && count == squares_on_tile.len() {
        for entity in squares_on_tile {
            let (entity, tile_pos, tile_entity) = entity;
            if let Ok((_, _, mut square)) = square_query.get_mut(entity) {
                if let Ok((_, mut sprite, _, _)) = tile_query.get_mut(tile_entity) {
                    sprite.color = Color::srgb(0., 1., 0.);
                }
                square.state = SquareState::Placing(tile_pos);
            }
        }
    }
}

pub(crate) fn place_figure(
    mut commands: Commands,
    mut square_query: Query<(Entity, &mut Transform, &mut Square)>,
    mut tile_query: Query<(&mut Tile, &GlobalTransform)>, // Query for tiles
) {
    for (entity, mut transform, mut square) in &mut square_query {
        if let SquareState::MustBePlaced(position) = square.state {
            square.state = SquareState::Placed(position);
            commands.entity(entity).remove_parent();
            *transform = Transform::from_translation(Vec3::new(position.x, position.y, 0.5));

            for (mut tile, tile_transform) in &mut tile_query {
                let tile_pos = tile_transform.translation().truncate();

                if tile_pos == position {
                    tile.is_free = false;
                    break;
                }
            }
        }
    }
}
