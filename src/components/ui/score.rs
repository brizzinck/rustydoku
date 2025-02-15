use bevy::prelude::*;

use crate::resource::score::Score;

#[derive(Component)]
pub struct ScoreText;

pub fn spawn_text_score(commands: &mut Commands, parent: Entity, assets: &Res<AssetServer>) {
    commands
        .spawn(Node {
            margin: UiRect {
                left: Val::Px(50.0),
                top: Val::Px(15.0),
                ..default()
            },
            ..default()
        })
        .with_children(|commands| {
            commands.spawn((
                Node {
                    margin: UiRect::all(Val::Auto),
                    ..default()
                },
                TextFont {
                    font_size: 37.0,
                    font: assets.load("rusty.otf"),
                    ..default()
                },
                Text::new("SCORE: 0"),
                ScoreText,
            ));
        })
        .set_parent(parent);
}

pub fn update_text_score(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    for mut span in &mut query {
        **span = format!("SCORE: {}", (*score).value);
    }
}
