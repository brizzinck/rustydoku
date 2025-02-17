use crate::{
    components::ui::{
        button_restart::handle_restart_button,
        game_over::spawn_game_over_panel,
        header::{score::update_text_score, spawn_header},
    },
    logic::{animation::game_over_panel::show_game_over_panel, ui::game_over_panel::hide_header},
    resource::score::update_max_score,
    states::gameplay::{game_over::StateGameOverPanel, StateGame},
};
use bevy::prelude::*;

pub struct RustydokuUIPlugin;

impl Plugin for RustydokuUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_header);
        app.add_systems(Update, (update_text_score, handle_restart_button));

        app.add_systems(
            OnEnter(StateGame::GameOver),
            (update_max_score, spawn_game_over_panel, hide_header).chain(),
        );

        app.add_systems(
            Update,
            show_game_over_panel.run_if(StateGameOverPanel::when_showing),
        );
    }
}
