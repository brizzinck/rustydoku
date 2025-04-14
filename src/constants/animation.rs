/// The threshold for detecting when a value is "close enough" in time-based comparisons.
///
/// Used primarily in animation systems to determine when an animation step is considered complete.
/// Helps avoid floating-point precision issues.
pub const ELAPSED_TIME: f32 = 0.00000001;

/// The threshold for detecting when a scale transformation is "close enough" to the target scale.
///
/// Commonly used in interpolation logic (e.g., `Vec3::lerp`) to determine if scaling animations
/// can be considered finished.
pub const ELAPSED_SCALE: f32 = 0.01;
