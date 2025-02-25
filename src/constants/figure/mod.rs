use bevy::prelude::*;

use super::map::TILE_SIZE;

pub mod assets;
pub mod placeholder;
pub mod spawn_zone;

pub const SQUARE_SIZE: f32 = TILE_SIZE;
pub const SQUARE_PLACED_POSITION: Vec3 = Vec3::new(0., 0., 0.5);
pub const MAX_FIGURE_SIZE: f32 = SQUARE_SIZE * 3.;
pub const MAX_FIGURE_USIZE_SCALED: usize = 3;
pub const FIGURE_POSITION_Z: f32 = 1.;
pub const FIGURE_DRAGGING_SCALE: f32 = 1.;
pub const FIGURE_IDEL_SCALE: f32 = 0.6;
pub const FIGURE_SPEED_TO_UPSCALE: f32 = 8.;
pub const FIGURE_UPSCALE_SPEED_INCREMENT_FACTOR_PER_FRAME: f32 = 1.;
pub const FIGURE_OFFSET_DRAGGING_Y: f32 = 55.;
pub const FIGURE_OFFSET_DRAGGING_Y_MULTIPLY: f32 = 10.;
pub const INTERACTIVE_ZONE_COLOR: Srgba = Srgba::new(0., 0., 0., 0.);
