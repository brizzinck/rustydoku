use bevy::prelude::*;

/// Width of the restart button in the header.
pub const HEADER_RESTART_BUTTON_WIDTH: Val = Val::Px(50.0);

/// Height of the restart button in the header.
pub const HEADER_RESTART_BUTTON_HEIGHT: Val = Val::Px(50.0);

/// Margin applied to the restart button in the header.
pub const HEADER_RESTART_BUTTON_MARGIN: UiRect = UiRect {
    left: Val::Auto,
    right: Val::Px(10.0),
    top: Val::Px(20.0),
    bottom: Val::Auto,
};

/// Width of the restart button in the game over panel.
pub const GAME_OVER_RESTART_BUTTON_WIDTH: Val = Val::Px(66.0);

/// Height of the restart button in the game over panel.
pub const GAME_OVER_RESTART_BUTTON_HEIGHT: Val = Val::Px(66.0);

/// Horizontal alignment of the restart button in the game over panel.
pub const GAME_OVER_RESTART_BUTTON_JUSTIFY: JustifyContent = JustifyContent::Center;
