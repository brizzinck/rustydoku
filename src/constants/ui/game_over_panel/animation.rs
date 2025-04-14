use super::GAME_OVER_PANEL_TOP_DEFAULT_VALUE;

/// The timer duration for the game over panel animation.
///
/// Defines how long the panel's animation lasts when transitioning.
pub const GAME_OVER_PANEL_ANIMATION_TIMER: f32 = 1.;

/// Default speed for the game over panel animation.
///
/// Used as the base speed for the panel's entrance or exit.
pub const GAME_OVER_PANEL_ANIMATION_SPEED_DEFAULT: f32 = 1.;

/// Multiplier for the speed when showing the game over panel.
///
/// Increases the speed during the panel’s entrance animation.
pub const GAME_OVER_PANEL_ANIMATION_SPEED_SHOW_MULTIPLIER: f32 = 1.;

/// Multiplier for the speed when hiding the game over panel.
///
/// Increases the speed during the panel’s exit animation.
pub const GAME_OVER_PANEL_ANIMATION_SPEED_HIDE_MULTIPLIER: f32 = 5.;

/// Final Y position for the game over panel, calculated based on the default position value.
///
/// The panel will animate to this position during the game over state.
pub static GAME_OVER_PANEL_TOP_END: f32 = GAME_OVER_PANEL_TOP_DEFAULT_VALUE / 2. - 10.;

/// Final reversed Y position for the game over panel.
///
/// Used when the panel is hidden or transitioning away.
pub static GAME_OVER_PANEL_TOP_END_REVERSED: f32 =
    GAME_OVER_PANEL_TOP_DEFAULT_VALUE - GAME_OVER_PANEL_TOP_END;
