use super::*;
use crate::resource::score::Score;

#[derive(Component)]
pub struct ScoreText;

pub fn spawn_text_score(commands: &mut Commands, parent: Entity, assets: &Res<AssetServer>) {
    commands
        .spawn(create_background())
        .with_children(|commands| {
            commands.spawn(create_score_text(assets));
        })
        .set_parent(parent);
}

pub fn update_text_score(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    for mut span in &mut query {
        **span = format!("{SCORE_TEXT_CONTENT}: {}", score.current_value);
    }
}

fn create_background() -> Node {
    Node {
        margin: SCORE_BACKGROUND_MARGIN,
        ..default()
    }
}

fn create_score_text(assets: &Res<AssetServer>) -> impl Bundle {
    (
        Node {
            margin: SCORE_TEXT_MARGIN,
            ..default()
        },
        TextFont {
            font_size: SCORE_FONT_SIZE,
            font: assets.load(SCORE_FONT_PATH),
            ..default()
        },
        Text::new(format!("{SCORE_TEXT_CONTENT}: 0")),
        TextColor(Srgba::rgb_u8(SCORE_TEXT_COLOR.0, SCORE_TEXT_COLOR.1, SCORE_TEXT_COLOR.1).into()),
        ScoreText,
    )
}
