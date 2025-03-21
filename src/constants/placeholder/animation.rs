use crate::constants::figure::animation::FIGURE_IDLE_SCALE;
use bevy::math::Vec3;

/// Final scale of a figure when it finishes returning to its placeholder.
pub const FIGURE_SCALE_LERPED: Vec3 = Vec3::splat(FIGURE_IDLE_SCALE);

/// Default placeholder scale after animations complete.
pub const PLACEHOLDER_SCALE_DEFAULT: Vec3 = Vec3::splat(1.);

/// Initial scale when placeholder starts its animation.
pub const PLACEHOLDER_SCALE_INITIAL: Vec3 = Vec3::splat(0.);

/// Peak scale during placeholder bounce animation.
pub const PLACEHOLDER_SCALE_PEAK: f32 = PLACEHOLDER_SCALE_DEFAULT.x + FIGURE_IDLE_SCALE * 0.2;

/// Speed of the placeholder bounce animation.
pub const PLACEHOLDER_ANIMATION_SPEED: f32 = 1.3;

/// Factor used to scale up the bounce speed based on current scale.
pub const PLACEHOLDER_SCALE_UP_FACTOR: f32 = 0.4;

/// Speed of figure returning to its placeholder after invalid placement.
pub const FIGURE_RETURN_SPEED_TO_PLACEHOLDER: f32 = 8.;
