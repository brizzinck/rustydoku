use bevy::prelude::*;

pub const GAME_OVER_BACKGROUND_NAME_HIERARCHY: &str = "GameOverBackground UI";

pub const GAME_OVER_BACKGROUND_WIDTH: Val = Val::Percent(60.);
pub const GAME_OVER_BACKGROUND_HEIGHT: Val = Val::Percent(125.);
pub const GAME_OVER_BACKGROUND_FLEX_DIRECTION: FlexDirection = FlexDirection::Column;
pub const GAME_OVER_BACKGROUND_JUSTIFY: JustifyContent = JustifyContent::Center;
pub const GAME_OVER_BACKGROUND_ALIGN: AlignItems = AlignItems::Center;
pub const GAME_OVER_BACKGROUND_MARGIN: UiRect = UiRect {
    left: Val::Auto,
    right: Val::Auto,
    top: Val::Auto,
    bottom: Val::Auto,
};
pub const GAME_OVER_BACKGROUND_ROW_GAP: Val = Val::Percent(5.);

pub const GAME_OVER_BACKGROUND_COLOR: Srgba = Srgba::new(1.0, 1.0, 1.0, 0.86);
