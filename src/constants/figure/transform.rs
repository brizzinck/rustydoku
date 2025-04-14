use crate::constants::square::transform::SQUARE_SIZE;
use bevy::color::Srgba;

/// Y-axis offset applied when a figure is being dragged by finger.
pub const FIGURE_DRAG_OFFSET_Y_FINGER: f32 = 75.;

/// Y-axis offset applied when a figure is being dragged by mouse.
pub const FIGURE_DRAG_OFFSET_Y_MOUSE: f32 = 55.;

/// Multiplier applied to offset Y-axis when mouse dragging based on the figure's bounding box.
pub const FIGURE_DRAG_OFFSET_Y_MIN_MULTIPLIER: f32 = 1.4;

/// Maximum figure size in world units, used for placeholder sizing or layout.
pub static MAX_FIGURE_SIZE: f32 = SQUARE_SIZE * 3.;

/// Maximum figure size as a `usize` (used in block grouping).
pub const MAX_FIGURE_USIZE_SCALED: usize = 3;

/// Z-index for figure entities, ensures figures render above tiles and placeholders.
pub const FIGURE_Z_POSITION: f32 = 1.;

/// Invisible interactive area used to detect interactions with figures.
pub const INTERACTIVE_ZONE_COLOR: Srgba = Srgba::new(0., 0., 0., 0.);
