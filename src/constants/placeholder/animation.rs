use crate::constants::figure::animation::FIGURE_IDLE_SCALE;
use bevy::math::Vec3;

pub const FIGURE_SCALE_LERPED: Vec3 = Vec3::splat(FIGURE_IDLE_SCALE);
pub const PLACEHOLDER_SCALE_DEFAULT: Vec3 = Vec3::splat(1.);
pub const PLACEHOLDER_SCALE_INITIAL: Vec3 = Vec3::splat(0.);
pub const PLACEHOLDER_SCALE_PEAK: f32 = PLACEHOLDER_SCALE_DEFAULT.x + FIGURE_IDLE_SCALE * 0.2;
pub const PLACEHOLDER_ANIMATION_SPEED: f32 = 1.3;
pub const PLACEHOLDER_SCALE_UP_FACTOR: f32 = 0.4;
pub const FIGURE_RETURN_SPEED_TO_PLACEHOLDER: f32 = 8.;
