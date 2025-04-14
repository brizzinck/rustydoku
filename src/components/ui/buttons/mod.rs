use crate::constants::ui::buttons::panel::*;
use bevy::prelude::*;
use button_audio::ButtonAudio;
use button_restart::ButtonRestart;

pub mod button_audio;
pub mod button_restart;

/// Component marker for the UI panel that holds control buttons (audio, restart)
#[derive(Component)]
pub(crate) struct ButtonsPanel;

impl ButtonsPanel {
    /// Spawns the buttons panel in the header section.
    ///
    /// This panel includes the restart and audio buttons, placed in the game UI header.
    ///
    /// # Parameters
    /// - `child_builder`: Bevy's child entity builder.
    /// - `assets`: Asset server reference for loading UI resources.
    pub(crate) fn spawn_header(child_builder: &mut ChildBuilder, assets: &AssetServer) {
        child_builder
            .spawn(ButtonsPanel::create_panel(
                BUTTONS_PANEL_MARGIN_HEADER,
                BUTTONS_PANEL_MAX_WIDTH_HEADER,
                BUTTONS_PANEL_MAX_HEIGHT_HEADER,
            ))
            .with_children(|panel| {
                ButtonRestart::spawn_in_header(panel, assets);

                ButtonAudio::spawn(panel, assets);
            });
    }

    /// Spawns the buttons panel in the game over screen.
    ///
    /// Used when the game ends and the UI shows game over options.
    ///
    /// # Parameters
    /// - `child_builder`: Bevy's child entity builder.
    /// - `assets`: Asset server reference for loading UI resources.
    pub(crate) fn spawn_game_over(child_builder: &mut ChildBuilder, assets: &AssetServer) {
        child_builder
            .spawn(ButtonsPanel::create_panel(
                BUTTONS_PANEL_MARGIN_GAME_OVER,
                BUTTONS_PANEL_MAX_WIDTH_GAME_OVER,
                BUTTONS_PANEL_MAX_HEIGHT_GAME_OVER,
            ))
            .with_children(|panel| {
                ButtonRestart::spawn_in_game_over(panel, assets);

                ButtonAudio::spawn(panel, assets);
            });
    }

    /// Creates a generic buttons panel node with configurable sizing and layout.
    ///
    /// # Parameters
    /// - `margin`: Margin around the panel.
    /// - `max_width`: Maximum width for the panel.
    /// - `max_height`: Maximum height for the panel.
    ///
    /// # Returns
    /// A Bevy node bundle for the buttons panel.
    fn create_panel(margin: UiRect, max_width: Val, max_height: Val) -> impl Bundle {
        (
            Name::new(BUTTONS_PANEL_NAME_HIERARCHY),
            Node {
                left: BUTTONS_PANEL_LEFT,
                right: BUTTONS_PANEL_RIGHT,
                bottom: BUTTONS_PANEL_BOTTOM,
                position_type: BUTTONS_PANEL_POSITION,
                top: BUTTONS_PANEL_TOP,
                width: BUTTONS_PANEL_WIDTH,
                height: BUTTONS_PANEL_HEIGHT,
                margin,
                max_width,
                max_height,
                justify_self: BUTTONS_PANEL_JUSTIFY,
                column_gap: BUTTONS_PANEL_COLUMN_GAP,
                ..default()
            },
            ButtonsPanel,
        )
    }
}
