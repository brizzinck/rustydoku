use crate::components::ui::header::score_text::HeaderCurrentScoreTextComponent;
use crate::constants::ui::header::score::*;
use crate::resource::score::ScoreResource;
use bevy::prelude::*;

impl HeaderCurrentScoreTextComponent {
    pub fn spawn(commands: &mut ChildBuilder, assets: &Res<AssetServer>) {
        commands
            .spawn(HeaderCurrentScoreTextComponent::create_background())
            .with_children(|commands| {
                commands.spawn(HeaderCurrentScoreTextComponent::create_score_text(assets));
            });
    }

    pub fn update(
        score: Res<ScoreResource>,
        mut query: Query<&mut Text, With<HeaderCurrentScoreTextComponent>>,
    ) {
        for mut span in &mut query {
            **span = format!("{HEADER_SCORE_TEXT_CONTENT}: {}", score.get_current_score());
        }
    }
}
