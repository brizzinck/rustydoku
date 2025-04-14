use crate::components::ui::game_over_panel::score_text::*;
use crate::constants::ui::game_over_panel::*;
use crate::resource::score::ScoreResource;
use bevy::prelude::*;

impl GameOverCurrentScoreTextComponent {
    /// Updates the current score text displayed in the Game Over panel.
    ///
    /// This function retrieves the current score from the [`ScoreResource`] and updates
    /// all `Text` components associated with the [`GameOverCurrentScoreTextComponent`]
    /// to reflect the latest value.
    ///
    /// # Parameters
    /// - `score`: The current score resource.
    /// - `query`: A query for text elements tagged with [`GameOverCurrentScoreTextComponent`].
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
    /// Updates the max score text displayed in the Game Over panel.
    ///
    /// This function retrieves the maximum score from the [`ScoreResource`] and updates
    /// all `Text` components associated with the [`GameOverMaxScoreTextComponent`]
    /// to display the highest score achieved.
    ///
    /// # Parameters
    /// - `score`: The current score resource.
    /// - `query`: A query for text elements tagged with [`GameOverMaxScoreTextComponent`].
    pub fn update(
        score: Res<ScoreResource>,
        mut query: Query<&mut Text, With<GameOverMaxScoreTextComponent>>,
    ) {
        for mut span in &mut query {
            **span = format!("{MAX_SCORE_TEXT_CONTENT}: {}", score.get_max_score());
        }
    }
}
