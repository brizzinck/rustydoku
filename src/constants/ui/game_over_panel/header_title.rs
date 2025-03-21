use bevy::prelude::*;

/// Text for the header title in the game over panel.
pub const HEADER_TITLE_TEXT: &str = "LOSS";

/// Font size for the header title in the game over panel.
pub const HEADER_TITLE_FONT_SIZE: f32 = 37.0;

/// Margin for the header title in the game over panel.
pub const HEADER_TITLE_MARGIN: UiRect = UiRect {
    left: Val::Auto,
    right: Val::Auto,
    top: Val::Percent(10.0),
    bottom: Val::Auto,
};

/// Color of the header title text.
pub const HEADER_TITLE_COLOR: Color = Color::srgb(0.9, 0.1, 0.1);
