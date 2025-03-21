use bevy::prelude::*;

/// Name used in the UI hierarchy for the game over background.
pub const GAME_OVER_BACKGROUND_NAME_HIERARCHY: &str = "GameOverBackground UI";

/// Width of the game over background relative to the window size (percentage).
pub const GAME_OVER_BACKGROUND_WIDTH: Val = Val::Percent(60.);

/// Height of the game over background relative to the window size (percentage).
pub const GAME_OVER_BACKGROUND_HEIGHT: Val = Val::Percent(125.);

/// The flex direction for the game over background.
pub const GAME_OVER_BACKGROUND_FLEX_DIRECTION: FlexDirection = FlexDirection::Column;

/// Justification of the game over background content.
pub const GAME_OVER_BACKGROUND_JUSTIFY: JustifyContent = JustifyContent::Center;

/// Alignment of items in the game over background.
pub const GAME_OVER_BACKGROUND_ALIGN: AlignItems = AlignItems::Center;

/// Margin applied to the game over background in the UI.
pub const GAME_OVER_BACKGROUND_MARGIN: UiRect = UiRect {
    left: Val::Auto,
    right: Val::Auto,
    top: Val::Auto,
    bottom: Val::Auto,
};

/// Row gap between elements inside the game over background.
pub const GAME_OVER_BACKGROUND_ROW_GAP: Val = Val::Percent(5.);

/// The background color of the game over panel, with some transparency.
pub const GAME_OVER_BACKGROUND_COLOR: Srgba = Srgba::new(1.0, 1.0, 1.0, 0.86);
