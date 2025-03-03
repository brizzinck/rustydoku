use bevy::prelude::*;

pub const SQUARE_NAME_HIERARCHY: &str = "square";

pub const SQUARE_ALPHA_DEFAULT: f32 = 1.;
pub const SQUARE_ALPHA_TARGET_WHEN_COMBO: f32 = 0.;
pub const SQUARE_ALPHA_SPEED_ANIMATION: f32 = 1.;
pub const SQUARE_FADE_OUT_SPEED_INCREMENT_FACTOR_PER_FRAME: f32 = 1.;
pub const SQUARE_COLOR_DEFAULT: Srgba = Srgba::new(1., 1., 1., SQUARE_ALPHA_DEFAULT);
