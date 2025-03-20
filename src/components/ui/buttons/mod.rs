use crate::constants::ui::buttons::panel::*;
use bevy::prelude::*;
use button_audio::AudioButton;
use button_restart::RestartButton;

pub mod button_audio;
pub mod button_restart;

#[derive(Component)]
pub(crate) struct ButtonsPanel;

impl ButtonsPanel {
    pub fn spawn_header(child_builder: &mut ChildBuilder, assets: &AssetServer) {
        child_builder
            .spawn(ButtonsPanel::create_panel(
                BUTTONS_PANEL_MARGIN_HEADER,
                BUTTONS_PANEL_BOTTOM_HEADER,
            ))
            .with_children(|panel| {
                RestartButton::spawn_in_header(panel, assets);

                AudioButton::spawn(panel, assets);
            });
    }

    pub fn spawn_game_over(child_builder: &mut ChildBuilder, assets: &AssetServer) {
        child_builder
            .spawn(ButtonsPanel::create_panel(
                BUTTONS_PANEL_MARGIN_GAME_OVER,
                BUTTONS_PANEL_BOTTOM_GAME_OVER,
            ))
            .with_children(|panel| {
                RestartButton::spawn_in_game_over(panel, assets);

                AudioButton::spawn(panel, assets);
            });
    }

    fn create_panel(margin: UiRect, bottom: Val) -> impl Bundle {
        (
            Name::new(BUTTONS_PANEL_NAME_HIERARCHY),
            Node {
                left: BUTTONS_PANEL_LEFT,
                right: BUTTONS_PANEL_RIGHT,
                bottom,
                position_type: BUTTONS_PANEL_POSITION,
                top: BUTTONS_PANEL_TOP,
                width: BUTTONS_PANEL_WIDTH,
                height: BUTTONS_PANEL_HEIGHT,
                margin,
                max_width: BUTTONS_PANEL_MAX_WIDTH,
                max_height: BUTTONS_PANEL_MAX_HEIGHT,
                justify_self: BUTTONS_PANEL_JUSTIFY,
                column_gap: BUTTONS_PANEL_COLUMN_GAP,
                ..default()
            },
        )
    }
}
