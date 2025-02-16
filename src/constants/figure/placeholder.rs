use super::{FIGURE_IDEL_SCALE, MAX_FIGURE_SIZE, SQUARE_SIZE};
use bevy::color::Srgba;

pub const PLACEHOLDER_NAME_HIERARCHY: &str = "Placeholder";

pub const PLACEHOLDER_SCALE: f32 = FIGURE_IDEL_SCALE;
pub const PLACEHOLDER_OFFSET: f32 = 10.;
pub const PLACEHOLDER_SIZE: f32 = (MAX_FIGURE_SIZE + PLACEHOLDER_OFFSET) * PLACEHOLDER_SCALE;
pub const PLACEHOLDER_COLOR: Srgba = Srgba::new(1., 1., 1., 0.45);
pub const PLACEHOLDER_POSITION: [(f32, f32); 3] = [
    (SQUARE_SIZE * 3.45, SQUARE_SIZE * -7.),
    (0., SQUARE_SIZE * -7.),
    (SQUARE_SIZE * -3.45, SQUARE_SIZE * -7.),
];
pub const PLACEHOLDER_POSITION_Z: f32 = 0.;
pub const FIGURE_SPEED_RETURN_TO_PLACEHOLDER: f32 = 8.;
