use crate::constants::ui::game_over_panel::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct GameOverCurrentScoreText;

impl GameOverCurrentScoreText {
    pub(crate) fn create_current_score(assets: &Res<AssetServer>) -> impl Bundle {
        (
            Name::new(SCORE_NAME_HIERARCHY),
            Text::new(format!("{SCORE_TEXT_CONTENT}: 0")),
            TextFont {
                font_size: SCORE_TEXT_FONT_SIZE,
                font: assets.load(GAME_OVER_SCORE_FONT_PATH),
                ..default()
            },
            TextColor(SCORE_TEXT_COLOR),
            GameOverCurrentScoreText,
        )
    }
}

#[derive(Component)]
pub struct GameOverMaxScoreText;

impl GameOverMaxScoreText {
    pub(crate) fn create_max_score(assets: &Res<AssetServer>) -> impl Bundle {
        (
            Name::new(MAX_SCORE_NAME_HIERARCHY),
            Text::new(format!("{MAX_SCORE_TEXT_CONTENT}: 0")),
            TextFont {
                font_size: MAX_SCORE_TEXT_FONT_SIZE,
                font: assets.load(GAME_OVER_MAX_SCORE_FONT_PATH),
                ..default()
            },
            TextColor(MAX_SCORE_TEXT_COLOR),
            GameOverMaxScoreText,
        )
    }
}
