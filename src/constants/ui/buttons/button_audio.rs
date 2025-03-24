use bevy::prelude::*;

/// Width of the audio button in the header section.
pub const HEADER_AUDIO_BUTTON_WIDTH: Val = Val::Px(50.);

/// Height of the audio button in the header section.
pub const HEADER_AUDIO_BUTTON_HEIGHT: Val = Val::Px(50.);

/// Margin applied to the audio button in the header.
pub const HEADER_AUDIO_BUTTON_MARGIN: UiRect = UiRect {
    left: Val::Auto,
    right: Val::Px(10.),
    top: Val::Px(20.),
    bottom: Val::Auto,
};

/// Width of the audio button in the game over panel.
pub const GAME_OVER_AUDIO_BUTTON_WIDTH: Val = Val::VMax(6.8);

/// Height of the audio button in the game over panel.
pub const GAME_OVER_AUDIO_BUTTON_HEIGHT: Val = Val::VMax(6.8);

/// Max width of the audio button in the game over panel.
pub const GAME_OVER_AUDIO_BUTTON_MAX_WIDTH: Val = Val::Px(80.);

/// Max height of the audio button in the game over panel.
pub const GAME_OVER_AUDIO_BUTTON_MAX_HEIGHT: Val = Val::Px(80.);

/// Horizontal alignment of the audio button in the game over panel.
pub const GAME_OVER_AUDIO_BUTTON_JUSTIFY: JustifyContent = JustifyContent::Center;
