use crate::components::ui::header::score_text::HeaderCurrentScoreTextComponent;
use crate::constants::ui::header::score::*;
use crate::resource::score::ScoreResource;
use bevy::prelude::*;

impl HeaderCurrentScoreTextComponent {
    /// Spawns the current score display in the header UI.
    ///
    /// # Parameters
    /// - `commands`: A mutable reference to a [`ChildBuilder`] used for spawning child UI entities.
    /// - `assets`: A reference to the [`AssetServer`] to load fonts or UI assets for the score text.
    pub fn spawn(commands: &mut ChildBuilder, assets: &Res<AssetServer>) {
        commands
            .spawn(HeaderCurrentScoreTextComponent::create_background())
            .with_children(|commands| {
                commands.spawn(HeaderCurrentScoreTextComponent::create_score_text(assets));
            });
    }

    /// Updates the displayed score in the header UI to match the current score.
    ///
    /// This function queries all text elements associated with [`HeaderCurrentScoreTextComponent`]
    /// and updates their text content to reflect the latest score from [`ScoreResource`].
    ///
    /// # Parameters
    /// - `score`: A resource that stores the current score value.
    /// - `query`: A query for accessing and updating [`Text`] components tagged with `HeaderCurrentScoreTextComponent`.
    pub fn update(
        score: Res<ScoreResource>,
        mut query: Query<&mut Text, With<HeaderCurrentScoreTextComponent>>,
    ) {
        for mut span in &mut query {
            **span = format!("{HEADER_SCORE_TEXT_CONTENT}: {}", score.get_current_score());
        }
    }
}
