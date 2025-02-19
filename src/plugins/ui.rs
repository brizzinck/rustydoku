use crate::{
    components::ui::{
        button_restart::RestartButton, game_over::GameOverPanel, header::score_text::ScoreText,
        header::HeaderUI,
    },
    logic::animation::game_over_panel::{
        hide_game_over_panel, set_hide_game_over_panel, set_show_game_over_panel,
        show_game_over_panel,
    },
    resource::score::Score,
    states::{gameplay::StateGame, ui::game_over_panel::StateGameOverPanel},
};
use bevy::prelude::*;

pub struct RustydokuUIPlugin;

impl Plugin for RustydokuUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (HeaderUI::spawn, GameOverPanel::spawn));
        app.add_systems(Update, (ScoreText::update, RestartButton::handle));

        app.add_systems(
            OnEnter(StateGame::GameOver),
            (
                Score::update_max_score,
                HeaderUI::hide,
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
