use bevy::prelude::*;

use crate::resource::score::Score;

#[derive(Component)]
pub struct ScoreText;

pub fn spawn_text_score(mut commands: Commands) {
    commands
        .spawn(Node {
            top: Val::Px(10.),
            left: Val::Percent(6.),
            ..default()
        })
        .with_children(|commands| {
            commands.spawn((
                Text::new("SCORE: "),
                TextFont {
                    font_size: 57.0,
                    ..default()
                },
            ));
            commands.spawn((
                Text::new("0"),
                TextFont {
                    font_size: 67.0,
                    ..default()
                },
                ScoreText,
            ));
        });
}

pub fn update_text_score(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    for mut span in &mut query {
        **span = format!("{}", score.value);
    }
}
