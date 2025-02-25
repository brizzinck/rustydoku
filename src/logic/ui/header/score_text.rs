use crate::components::ui::header::score_text::HeaderCurrentScoreText;
use crate::constants::ui::header::score::*;
use crate::resource::score::Score;
use bevy::prelude::*;

impl HeaderCurrentScoreText {
    pub fn spawn(commands: &mut Commands, parent: Entity, assets: &Res<AssetServer>) {
        commands
            .spawn(HeaderCurrentScoreText::create_background())
            .with_children(|commands| {
                commands.spawn(HeaderCurrentScoreText::create_score_text(assets));
            })
            .set_parent(parent);
    }

    pub fn update(score: Res<Score>, mut query: Query<&mut Text, With<HeaderCurrentScoreText>>) {
        for mut span in &mut query {
            **span = format!("{SCORE_TEXT_CONTENT}: {}", score.get_current_score());
        }
    }
}
