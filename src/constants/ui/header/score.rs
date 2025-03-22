use bevy::prelude::*;

/// The margin of the score header.
pub const HEADER_SCORE_BACKGROUND_MARGIN: UiRect = UiRect {
    left: Val::Px(10.),
    top: Val::Px(17.),
    bottom: Val::Px(0.),
    right: Val::Px(0.),
};

/// The width of the score header.
pub const HEADER_SCORE_MARGIN: UiRect = UiRect::all(Val::Auto);

/// The width of the score header.
pub const HEADER_SCORE_FONT_SIZE: f32 = 47.;

/// The font path for the score header.
pub const HEADER_SCORE_TEXT_CONTENT: &str = "SCORE";

/// The color of the score header.
pub const HEADER_SCORE_TEXT_COLOR: (u8, u8, u8) = (177, 177, 177);
