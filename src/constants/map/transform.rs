use std::ops::RangeInclusive;

/// The total size of the map (width and height in tiles).
pub const MAP_SIZE: i8 = 9;

/// Map size as `usize`, used for indexing arrays.
pub const MAP_SIZE_USIZE: usize = MAP_SIZE as usize;

/// Map size as `f32`, used for calculations in world space.
pub const MAP_SIZE_F32: f32 = MAP_SIZE as f32;

/// The size (width and height) of an individual tile, in world units.
pub const TILE_SIZE: f32 = 40.0;

/// Half the size of the entire map in world units, used for centering.
pub static HALF_MAP_SIZE: f32 = (MAP_SIZE_F32 * TILE_SIZE) / 2.0;

/// Half the size of a single tile.
pub static HALF_TILE_SIZE: f32 = TILE_SIZE / 2.0;

/// The Z-layer position for map tiles (used for rendering depth).
pub const TILE_Z_POSITION: f32 = 0.;

/// The inclusive range of positions to spawn map tiles, centered around 0.
///
/// Example: if MAP_SIZE = 9, this range is -4..=4
pub const MAP_SPAWN_POSITIOM: RangeInclusive<i8> = (-MAP_SIZE / 2)..=(MAP_SIZE / 2);
