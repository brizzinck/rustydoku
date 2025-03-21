/// The base alpha fading speed for a square during the fade-out animation.
///
/// Used in the interpolation calculation for transparency.
pub const SQUARE_ANIMATION_ALPHA_SPEED: f32 = 1.;

/// The per-frame increment used to speed up fade-out based on the current alpha.
///
/// Helps create a non-linear fade effect where the square fades faster as it becomes more transparent.
pub const SQUARE_FADE_OUT_SPEED_INCREMENT_PER_FRAME: f32 = 4.;
