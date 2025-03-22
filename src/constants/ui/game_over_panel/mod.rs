use bevy::prelude::*;

pub mod animation;
pub mod assets;
pub mod background;
pub mod header_title;
pub mod score;

pub use animation::*;
pub use assets::*;
pub use background::*;
pub use header_title::*;
pub use score::*;

/// Name used in the UI hierarchy for the game over panel.
pub const GAME_OVER_PANEL_NAME_HIERARCHY: &str = "GameOverPanel UI";

/// Positioning of the game over panel (left position).
pub const GAME_OVER_PANEL_LEFT: Val = Val::Auto;

/// Positioning of the game over panel (right position).
pub const GAME_OVER_PANEL_RIGHT: Val = Val::Auto;

/// Positioning of the game over panel (bottom position).
pub const GAME_OVER_PANEL_BOTTOM: Val = Val::Auto;

/// The positioning mode of the game over panel.
pub const GAME_OVER_PANEL_POSITION: PositionType = PositionType::Absolute;

/// Default top position of the game over panel.
pub const GAME_OVER_PANEL_TOP_DEFAULT_VALUE: f32 = 120.;

/// The top position of the game over panel in percentage (relative to window).
pub const GAME_OVER_PANEL_TOP_DEFAULT: Val = Val::Percent(GAME_OVER_PANEL_TOP_DEFAULT_VALUE);

/// Width of the game over panel as a percentage of the window width.
pub const GAME_OVER_PANEL_WIDTH: Val = Val::VMin(80.);

/// Height of the game over panel as a percentage of the window height.
pub const GAME_OVER_PANEL_HEIGHT: Val = Val::VMin(40.);

/// Margin for the game over panel.
pub const GAME_OVER_PANEL_MARGIN: UiRect = UiRect {
    left: Val::Percent(0.),
    right: Val::Percent(0.),
    top: Val::Px(0.),
    bottom: Val::Px(0.),
};

/// Maximum width of the game over panel.
pub const GAME_OVER_PANEL_MAX_WIDTH: Val = Val::VMax(50.);

/// Maximum height of the game over panel.
pub const GAME_OVER_PANEL_MAX_HEIGHT: Val = Val::VMax(25.);

/// Justification for the game over panel's position.
pub const GAME_OVER_PANEL_JUSTIFY: JustifySelf = JustifySelf::Center;
