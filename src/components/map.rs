use bevy::{
    app::{Plugin, Startup},
    prelude::*,
};
use std::ops::RangeInclusive;

const MAP_SIZE: i8 = 9;
pub const TILE_SIZE: f32 = 40.0;
const MAP_SPAWN_POS: RangeInclusive<i8> = (-MAP_SIZE / 2)..=(MAP_SIZE / 2);
const COLOR_LIGHT: Color = Color::srgb(0.9, 0.9, 0.9);
const COLOR_DARK: Color = Color::srgb(0.5, 0.5, 0.5);

pub struct Map;

#[derive(Component)]
pub struct Tile {
    pub(super) default_color: Color,
    #[allow(dead_code)]
    pub(super) is_free: bool,
}

impl Plugin for Map {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, generate_map);
    }
}

fn generate_map(mut commands: Commands) {
    for x in MAP_SPAWN_POS {
        for y in MAP_SPAWN_POS {
            let color = if (x + y) % 2 == 0 {
                COLOR_DARK
            } else {
                COLOR_LIGHT
            };

            commands.spawn((
                Sprite {
                    color,
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                Transform::from_xyz(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.),
                Tile {
                    default_color: color,
                    is_free: true,
                },
            ));
        }
    }
}
