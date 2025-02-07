use bevy::prelude::*;

use crate::components::map::{Tile, TILE_SIZE};

pub const SQUARE_SIZE: f32 = TILE_SIZE;

#[derive(Default, PartialEq, Debug)]
pub enum State {
    #[default]
    Idle,
    Dragging,
    Placing(Vec2),
    MustBePlaced(Vec2),
    Placed(Vec2),
}

#[derive(Component, Default)]
pub struct Square {
    pub(super) state: State,
}

pub(super) fn spawn(commands: &mut Commands, parent: Entity, position: Vec2) {
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

pub(super) fn highlight_tile(
    mut tile_query: Query<(&Tile, &mut Sprite, &GlobalTransform)>,
    mut square_query: Query<(&mut Square, &GlobalTransform)>,
) {
    for (tile, mut sprite, tile_transform) in &mut tile_query {
        let tile_pos = tile_transform.translation().truncate();
        let tile_grid_x = (tile_pos.x / TILE_SIZE).round() as i32;
        let tile_grid_y = (tile_pos.y / TILE_SIZE).round() as i32;

        let mut is_over_tile = false;

        for (mut square, square_transform) in &mut square_query {
            if matches!(square.state, State::Idle | State::Placed(_)) {
                continue;
            }

            let square_pos = square_transform.translation().truncate();
            let square_grid_x = (square_pos.x / TILE_SIZE).round() as i32;
            let square_grid_y = (square_pos.y / TILE_SIZE).round() as i32;

            if tile_grid_x == square_grid_x && tile_grid_y == square_grid_y {
                is_over_tile = true;

                if matches!(square.state, State::Dragging | State::Placing(_)) {
                    square.state = State::Placing(tile_pos);
                }

                break;
            }
        }

        sprite.color = if is_over_tile {
            Color::srgb(0.0, 1.0, 0.0)
        } else {
            tile.default_color
        };
    }
}

pub(super) fn place_figure(
    mut commands: Commands,
    mut square_query: Query<(Entity, &mut Transform, &mut Square)>,
    mut tile_query: Query<(&mut Tile, &GlobalTransform)>, // Query for tiles
) {
    for (entity, mut transform, mut square) in &mut square_query {
        if let State::MustBePlaced(position) = square.state {
            square.state = State::Placed(position);
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
