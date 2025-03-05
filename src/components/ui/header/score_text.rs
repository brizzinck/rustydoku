use super::*;

#[derive(Component)]
pub struct HeaderCurrentScoreText;

impl HeaderCurrentScoreText {
    pub(crate) fn create_background() -> Node {
        Node {
            margin: HEADER_SCORE_BACKGROUND_MARGIN,
            ..default()
        }
    }

    pub(crate) fn create_score_text(assets: &Res<AssetServer>) -> impl Bundle {
        (
            Node {
                margin: HEADER_SCORE_MARGIN,
                ..default()
            },
            TextFont {
                font_size: HEADER_SCORE_FONT_SIZE,
                font: assets.load(HEADER_SCORE_FONT_PATH),
                ..default()
            },
            Text::new(format!("{HEADER_SCORE_TEXT_CONTENT}: 0")),
            TextColor(
                Srgba::rgb_u8(
                    HEADER_SCORE_TEXT_COLOR.0,
                    HEADER_SCORE_TEXT_COLOR.1,
                    HEADER_SCORE_TEXT_COLOR.1,
                )
                .into(),
            ),
            HeaderCurrentScoreText,
        )
    }
}
