use crate::components::ui::game_over_panel::score_text::*;
use crate::constants::ui::game_over_panel::*;
use crate::resource::score::ScoreResource;
use bevy::prelude::*;

impl GameOverCurrentScoreTextComponent {
    pub fn update(
        score: Res<ScoreResource>,
        mut query: Query<&mut Text, With<GameOverCurrentScoreTextComponent>>,
    ) {
        for mut span in &mut query {
            **span = format!("{SCORE_TEXT_CONTENT}: {}", score.get_current_score());
        }
    }
}

impl GameOverMaxScoreTextComponent {
    pub fn update(
        score: Res<ScoreResource>,
        mut query: Query<&mut Text, With<GameOverMaxScoreTextComponent>>,
    ) {
        for mut span in &mut query {
            **span = format!("{MAX_SCORE_TEXT_CONTENT}: {}", score.get_max_score());
        }
    }
}
