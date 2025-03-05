use bevy::prelude::*;

pub const HEADER_SCORE_BACKGROUND_MARGIN: UiRect = UiRect {
    left: Val::Px(10.),
    top: Val::Px(17.),
    bottom: Val::Px(0.),
    right: Val::Px(0.),
};
pub const HEADER_SCORE_MARGIN: UiRect = UiRect::all(Val::Auto);
pub const HEADER_SCORE_FONT_SIZE: f32 = 47.;
pub const HEADER_SCORE_TEXT_CONTENT: &str = "SCORE";
pub const HEADER_SCORE_TEXT_COLOR: (u8, u8, u8) = (168, 67, 67);
