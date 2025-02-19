use crate::{
    components::ui::{
        button_restart::handle_restart_button,
        game_over::spawn_game_over_panel,
        header::{score::update_text_score, spawn_header},
    },
    logic::{
        animation::game_over_panel::{
            hide_game_over_panel, set_hide_game_over_panel, set_show_game_over_panel,
            show_game_over_panel,
        },
        ui::game_over_panel::hide_header,
    },
    resource::score::Score,
    states::{gameplay::StateGame, ui::game_over_panel::StateGameOverPanel},
};
use bevy::prelude::*;

pub struct RustydokuUIPlugin;

impl Plugin for RustydokuUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_header, spawn_game_over_panel));
        app.add_systems(Update, (update_text_score, handle_restart_button));

        app.add_systems(
            OnEnter(StateGame::GameOver),
            (
                Score::update_max_score,
                hide_header,
                set_show_game_over_panel,
            )
                .chain(),
        );

        app.add_systems(OnExit(StateGame::GameOver), set_hide_game_over_panel);

        app.add_systems(
            Update,
            show_game_over_panel.run_if(StateGameOverPanel::when_showing),
        );

        app.add_systems(
            Update,
            hide_game_over_panel.run_if(StateGameOverPanel::when_hidding),
        );
    }
}
