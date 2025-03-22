use super::score_text::*;
use crate::{
    components::ui::buttons::ButtonsPanel,
    constants::ui::{game_over_panel::*, DynamicText},
};
use bevy::prelude::*;

/// UI component for the game over panel.
///
/// This component holds the animation state and logic for displaying
/// the game over screen, including the header, score display, and control buttons.
#[derive(Component)]
pub struct GameOverPanelComponent {
    /// Timer for managing show/hide animations.
    pub timer: Timer,

    /// Speed factor for animation transitions.
    pub speed: f32,
}

impl GameOverPanelComponent {
    /// Spawns the complete game over panel UI hierarchy into the scene.
    ///
    /// # Parameters
    /// - `commands`: Bevy command buffer used for spawning entities.
    /// - `assets`: Handle to the `AssetServer` used to load textures and fonts.
    pub fn spawn(mut commands: Commands, assets: Res<AssetServer>) {
        commands
            .spawn(GameOverPanelComponent::create_panel())
            .with_children(|parent| {
                parent
                    .spawn(GameOverPanelComponent::create_background(&assets))
                    .with_children(|panel| {
                        panel.spawn(GameOverPanelComponent::create_header_title(&assets));

                        panel.spawn(GameOverMaxScoreTextComponent::create_max_score(&assets));

                        panel.spawn(GameOverCurrentScoreTextComponent::create_current_score(
                            &assets,
                        ));

                        ButtonsPanel::spawn_game_over(panel, &assets);
                    });
            });
    }

    /// Creates the outer UI node container for the panel, including animation config.
    ///
    /// # Returns
    /// A bundle representing the root container of the game over UI.
    fn create_panel() -> impl Bundle {
        (
            Name::new(GAME_OVER_PANEL_NAME_HIERARCHY),
            Node {
                left: GAME_OVER_PANEL_LEFT,
                right: GAME_OVER_PANEL_RIGHT,
                bottom: GAME_OVER_PANEL_BOTTOM,
                position_type: GAME_OVER_PANEL_POSITION,
                top: GAME_OVER_PANEL_TOP_DEFAULT,
                width: GAME_OVER_PANEL_WIDTH,
                height: GAME_OVER_PANEL_HEIGHT,
                margin: GAME_OVER_PANEL_MARGIN,
                max_width: GAME_OVER_PANEL_MAX_WIDTH,
                max_height: GAME_OVER_PANEL_MAX_HEIGHT,
                justify_self: GAME_OVER_PANEL_JUSTIFY,
                ..default()
            },
            GameOverPanelComponent {
                timer: Timer::from_seconds(GAME_OVER_PANEL_ANIMATION_TIMER, TimerMode::Once),
                speed: GAME_OVER_PANEL_ANIMATION_SPEED_DEFAULT,
            },
        )
    }

    /// Creates the background visual node for the game over panel.
    ///
    /// # Parameters
    /// - `assets`: AssetServer handle for loading images.
    ///
    /// # Returns
    /// A bundle including the panel background.
    fn create_background(assets: &Res<AssetServer>) -> impl Bundle {
        (
            Name::new(GAME_OVER_BACKGROUND_NAME_HIERARCHY),
            Node {
                width: GAME_OVER_BACKGROUND_WIDTH,
                height: GAME_OVER_BACKGROUND_HEIGHT,
                flex_direction: GAME_OVER_BACKGROUND_FLEX_DIRECTION,
                justify_content: GAME_OVER_BACKGROUND_JUSTIFY,
                align_items: GAME_OVER_BACKGROUND_ALIGN,
                margin: GAME_OVER_BACKGROUND_MARGIN,
                row_gap: GAME_OVER_BACKGROUND_ROW_GAP,
                ..default()
            },
            ImageNode {
                image: assets.load(GAME_OVER_BACKGROUND_PATH),
                color: GAME_OVER_BACKGROUND_COLOR.into(),
                ..default()
            },
        )
    }

    /// Creates the header title (e.g., "LOSS") displayed at the top of the panel.
    ///
    /// # Parameters
    /// - `assets`: AssetServer handle to load the font.
    ///
    /// # Returns
    /// A bundle representing the stylized header title.
    fn create_header_title(assets: &Res<AssetServer>) -> impl Bundle {
        (
            Node {
                margin: HEADER_TITLE_MARGIN,
                ..default()
            },
            Text::new(HEADER_TITLE_TEXT),
            TextFont {
                font_size: HEADER_TITLE_FONT_SIZE,
                font: assets.load(GAME_OVER_HEADER_FONT_PATH),
                ..default()
            },
            TextColor(HEADER_TITLE_COLOR),
            DynamicText::new(
                HEADER_TITLE_FONT_SIZE,
                HEADER_TITLE_FACTOR_FONT_SIZED,
                HEADER_TITLE_MAX_FONT_SIZE,
            ),
        )
    }
}
