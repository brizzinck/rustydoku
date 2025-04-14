use bevy::prelude::*;

/// Width of the restart button in the header.
pub const HEADER_RESTART_BUTTON_WIDTH: Val = Val::Px(50.);

/// Height of the restart button in the header.
pub const HEADER_RESTART_BUTTON_HEIGHT: Val = Val::Px(50.);

/// Margin applied to the restart button in the header.
pub const HEADER_RESTART_BUTTON_MARGIN: UiRect = UiRect {
    left: Val::Auto,
    right: Val::Px(10.),
    top: Val::Px(20.),
    bottom: Val::Auto,
};

/// Width of the restart button in the game over panel.
pub const GAME_OVER_RESTART_BUTTON_WIDTH: Val = Val::Auto;

/// Height of the restart button in the game over panel.
pub const GAME_OVER_RESTART_BUTTON_HEIGHT: Val = Val::Auto;

/// Max width of the restart button in the game over panel.
pub const GAME_OVER_RESTART_BUTTON_MAX_WIDTH: Val = Val::VMax(6.8);

/// Max height of the restart button in the game over panel.
pub const GAME_OVER_RESTART_BUTTON_MAX_HEIGHT: Val = Val::VMax(6.8);

/// Horizontal alignment of the restart button in the game over panel.
pub const GAME_OVER_RESTART_BUTTON_JUSTIFY: JustifyContent = JustifyContent::Center;
