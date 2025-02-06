use bevy::prelude::*;

use crate::components::map::{Tile, TILE_SIZE};

pub const SQUARE_SIZE: f32 = TILE_SIZE;

#[derive(Component)]
pub struct Square;

pub(super) fn spawn(commands: &mut Commands, parent: Entity, position: Vec2) {
    commands
        .spawn((
            Sprite {
                custom_size: Some(Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                ..Default::default()
            },
            Transform::from_xyz(position.x * SQUARE_SIZE, position.y * SQUARE_SIZE, 1.0),
            Square,
        ))
        .set_parent(parent);
}

pub(super) fn hightligh_tile(
    mut tile_query: Query<(&Tile, &mut Sprite, &GlobalTransform)>,
    square_query: Query<&GlobalTransform, With<Square>>,
) {
    for (tile, mut sprite, tile_transform) in &mut tile_query {
        let tile_pos = tile_transform.translation().truncate();
        let tile_grid_x = (tile_pos.x / TILE_SIZE).round() as i32;
        let tile_grid_y = (tile_pos.y / TILE_SIZE).round() as i32;

        let mut is_over_tile = false;

        for square_transform in &square_query {
            let square_pos = square_transform.translation().truncate();
            let square_grid_x = (square_pos.x / TILE_SIZE).round() as i32;
            let square_grid_y = (square_pos.y / TILE_SIZE).round() as i32;

            if tile_grid_x == square_grid_x && tile_grid_y == square_grid_y {
                is_over_tile = true;
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
