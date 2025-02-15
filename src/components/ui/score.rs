use bevy::prelude::*;

use crate::resource::score::Score;

#[derive(Component)]
pub struct ScoreText;

pub fn spawn_text_score(commands: &mut Commands, parent: Entity, assets: &Res<AssetServer>) {
    commands
        .spawn(Node {
            margin: UiRect {
                left: Val::Px(10.0),
                top: Val::Px(17.),
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
                    font_size: 47.0,
                    font: assets.load("rusty.otf"),
                    ..default()
                },
                Text::new("SCORE: 0"),
                TextColor(Srgba::rgb_u8(168, 67, 67).into()),
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
