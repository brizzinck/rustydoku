use bevy::prelude::*;

pub const HEADER_RESTART_BUTTON_WIDTH: Val = Val::Px(50.0);
pub const HEADER_RESTART_BUTTON_HEIGHT: Val = Val::Px(50.0);
pub const HEADER_RESTART_BUTTON_MARGIN: UiRect = UiRect {
    left: Val::Auto,
    right: Val::Px(10.0),
    top: Val::Px(20.0),
    bottom: Val::Auto,
};

pub const GAME_OVER_RESTART_BUTTON_WIDTH: Val = Val::Px(66.0);
pub const GAME_OVER_RESTART_BUTTON_HEIGHT: Val = Val::Px(66.0);
pub const GAME_OVER_RESTART_BUTTON_JUSTIFY: JustifyContent = JustifyContent::Center;
