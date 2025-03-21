use crate::constants::map::TILE_SIZE;
use bevy::{color::Srgba, math::Vec3};

/// The size of a square, derived from the tile size to ensure consistency on the grid.
pub const SQUARE_SIZE: f32 = TILE_SIZE;

/// Default local position for a square when placed into a tile (Z-layered).
pub const SQUARE_PLACED_POSITION: Vec3 = Vec3::new(0., 0., 0.5);

/// The default alpha value (fully visible) for a square.
pub const SQUARE_ALPHA_DEFAULT: f32 = 1.;

/// The target alpha value used when a square is fading out after a combo.
///
/// Represents full transparency.
pub const SQUARE_ALPHA_TARGET_COMBO: f32 = 0.;

/// Default color for squares, with full opacity.
pub const SQUARE_COLOR_DEFAULT: Srgba = Srgba::new(1., 1., 1., SQUARE_ALPHA_DEFAULT);
