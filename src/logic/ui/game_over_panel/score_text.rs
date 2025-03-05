use crate::components::ui::game_over_panel::score_text::*;
use crate::constants::ui::game_over_panel::*;
use crate::resource::score::Score;
use bevy::prelude::*;

impl GameOverCurrentScoreText {
    pub fn update(score: Res<Score>, mut query: Query<&mut Text, With<GameOverCurrentScoreText>>) {
        for mut span in &mut query {
            **span = format!("{SCORE_TEXT_CONTENT}: {}", score.get_current_score());
        }
    }
}

impl GameOverMaxScoreText {
    pub fn update(score: Res<Score>, mut query: Query<&mut Text, With<GameOverMaxScoreText>>) {
        for mut span in &mut query {
            **span = format!("{MAX_SCORE_TEXT_CONTENT}: {}", score.get_max_score());
        }
    }
}
