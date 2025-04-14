use crate::constants::{
    figure::{animation::FIGURE_IDLE_SCALE, transform::MAX_FIGURE_SIZE},
    square::transform::SQUARE_SIZE,
};
use bevy::{color::Srgba, math::Vec2};

/// The default color (with alpha) of the placeholder background.
pub const PLACEHOLDER_COLOR: Srgba = Srgba::new(1., 1., 1., 0.45);

/// The margin around the placeholder, used to calculate total size.
pub const PLACEHOLDER_MARGIN: f32 = 10.;

/// The computed size of the placeholder based on max figure size and idle scale.
pub const PLACEHOLDER_SIZE: Vec2 =
    Vec2::splat((MAX_FIGURE_SIZE + PLACEHOLDER_MARGIN) * FIGURE_IDLE_SCALE);

/// Predefined positions for each placeholder on screen (typically 3 for game logic).
pub const PLACEHOLDER_POSITIONS: [(f32, f32); 3] = [
    (SQUARE_SIZE * 3.45, SQUARE_SIZE * -7.),
    (0., SQUARE_SIZE * -7.),
    (SQUARE_SIZE * -3.45, SQUARE_SIZE * -7.),
];

/// Z-layer for rendering the placeholder behind other elements.
pub const PLACEHOLDER_Z_POSITION: f32 = 0.;
