use bevy::color::Color;
use std::ops::RangeInclusive;

pub mod assets;

pub const MAP_SIZE: i8 = 9;
pub const MAP_USIZE: usize = MAP_SIZE as usize;
pub const MAP_FSIZE: f32 = MAP_SIZE as f32;
pub const TILE_SIZE: f32 = 40.0;
pub const MAP_SPAWN_POS: RangeInclusive<i8> = (-MAP_SIZE / 2)..=(MAP_SIZE / 2);
pub const COLOR_LIGHT: Color = Color::srgb(0.67, 0.67, 0.67);
pub const COLOR_DARK: Color = Color::srgb(0.53, 0.53, 0.53);
