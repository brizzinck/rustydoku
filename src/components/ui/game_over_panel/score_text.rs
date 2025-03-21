use crate::constants::ui::game_over_panel::*;
use bevy::prelude::*;

/// UI component for displaying the current score in the game over panel.
#[derive(Component)]
pub struct GameOverCurrentScoreTextComponent;

impl GameOverCurrentScoreTextComponent {
    /// Constructs the current score text node.
    ///
    /// This displays the player's score at the end of the game.
    ///
    /// # Parameters
    /// - `assets`: Reference to Bevy's `AssetServer` for loading fonts.
    ///
    /// # Returns
    /// A bundle representing the text node for the current score.
    pub(crate) fn create_current_score(assets: &Res<AssetServer>) -> impl Bundle {
        (
            Name::new(SCORE_NAME_HIERARCHY),
            Text::new(format!("{SCORE_TEXT_CONTENT}: 0")),
            TextFont {
                font_size: SCORE_TEXT_FONT_SIZE,
                font: assets.load(GAME_OVER_SCORE_FONT_PATH),
                ..default()
            },
            TextColor(SCORE_TEXT_COLOR),
            GameOverCurrentScoreTextComponent,
        )
    }
}

/// UI component for displaying the max (high) score in the game over panel.
#[derive(Component)]
pub struct GameOverMaxScoreTextComponent;

impl GameOverMaxScoreTextComponent {
    /// Constructs the max score text node.
    ///
    /// This shows the best score the player has ever achieved.
    ///
    /// # Parameters
    /// - `assets`: Reference to Bevy's `AssetServer` for loading fonts.
    ///
    /// # Returns
    /// A bundle representing the text node for the max score.
    pub(crate) fn create_max_score(assets: &Res<AssetServer>) -> impl Bundle {
        (
            Name::new(MAX_SCORE_NAME_HIERARCHY),
            Text::new(format!("{MAX_SCORE_TEXT_CONTENT}: 0")),
            TextFont {
                font_size: MAX_SCORE_TEXT_FONT_SIZE,
                font: assets.load(GAME_OVER_MAX_SCORE_FONT_PATH),
                ..default()
            },
            TextColor(MAX_SCORE_TEXT_COLOR),
            GameOverMaxScoreTextComponent,
        )
    }
}
