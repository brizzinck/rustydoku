use super::*;

#[derive(Component)]
pub struct HeaderCurrentScoreText;

impl HeaderCurrentScoreText {
    pub(crate) fn create_background() -> Node {
        Node {
            margin: SCORE_BACKGROUND_MARGIN,
            ..default()
        }
    }

    pub(crate) fn create_score_text(assets: &Res<AssetServer>) -> impl Bundle {
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
            TextColor(
                Srgba::rgb_u8(SCORE_TEXT_COLOR.0, SCORE_TEXT_COLOR.1, SCORE_TEXT_COLOR.1).into(),
            ),
            HeaderCurrentScoreText,
        )
    }
}
