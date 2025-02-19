use crate::constants::ui::header::score::*;
use crate::{components::ui::header::score_text::ScoreText, resource::score::Score};
use bevy::prelude::*;

impl ScoreText {
    pub fn spawn(commands: &mut Commands, parent: Entity, assets: &Res<AssetServer>) {
        commands
            .spawn(ScoreText::create_background())
            .with_children(|commands| {
                commands.spawn(ScoreText::create_score_text(assets));
            })
            .set_parent(parent);
    }

    pub fn update(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
        for mut span in &mut query {
            **span = format!("{SCORE_TEXT_CONTENT}: {}", score.current_value);
        }
    }
}
