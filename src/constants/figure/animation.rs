use bevy::math::Vec3;

/// Upscale speed used when dragging a figure.
pub const FIGURE_SPEED_UPSCALE: f32 = 8.;

/// Factor added to scale speed per frame.
pub const FIGURE_UPSCALE_SPEED_INCREMENT_PER_FRAME: f32 = 1.;

/// Speed for spawning animation scaling up the figure.
pub const FIGURE_SPAWN_UPSCALE_SPEED: f32 = 5.;

/// Scale applied when a figure is being dragged.
pub const FIGURE_DRAGGING_SCALE: f32 = 1.;

/// Default idle scale of a figure.
pub const FIGURE_IDLE_SCALE: f32 = 0.6;

/// Default idle scale as a 3D vector.
pub const FIGURE_IDLE_SCALE_VEC3: Vec3 = Vec3::splat(FIGURE_IDLE_SCALE);
