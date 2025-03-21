use super::*;

/// Component for displaying the current score in the game header.
#[derive(Component)]
pub struct HeaderCurrentScoreTextComponent;

impl HeaderCurrentScoreTextComponent {
    /// Creates the background node for the score display.
    ///
    /// # Returns
    /// A node with the margin set to the score background margin.
    pub(crate) fn create_background() -> Node {
        Node {
            margin: HEADER_SCORE_BACKGROUND_MARGIN,
            ..default()
        }
    }

    /// Creates the text bundle to display the current score.
    ///
    /// # Parameters
    /// - `assets`: Reference to Bevy's `AssetServer` to load the score font.
    ///
    /// # Returns
    /// A bundle containing the node, text, font, color, and this component.
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
            HeaderCurrentScoreTextComponent,
        )
    }
}
