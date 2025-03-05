use bevy::{color::Srgba, math::Vec2};

use crate::constants::{
    figure::{animation::FIGURE_IDEL_SCALE, transform::MAX_FIGURE_SIZE},
    square::transform::SQUARE_SIZE,
};

pub const PLACEHOLDER_COLOR: Srgba = Srgba::new(1., 1., 1., 0.45);
pub const PLACEHOLDER_OFFSET: f32 = 10.;
pub const PLACEHOLDER_SIZE: Vec2 =
    Vec2::splat((MAX_FIGURE_SIZE + PLACEHOLDER_OFFSET) * FIGURE_IDEL_SCALE);
pub const PLACEHOLDER_POSITION: [(f32, f32); 3] = [
    (SQUARE_SIZE * 3.45, SQUARE_SIZE * -7.),
    (0., SQUARE_SIZE * -7.),
    (SQUARE_SIZE * -3.45, SQUARE_SIZE * -7.),
];
pub const PLACEHOLDER_POSITION_Z: f32 = 0.;
