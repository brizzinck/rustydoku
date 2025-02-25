use crate::{
    components::ui::header::HeaderUI,
    logic::{
        animation::game_over_panel::set_hide_game_over_panel,
        gameplay::figure_spawner::{clear_figures, hide_figures, respawn_figures, show_figures},
    },
    resource::{map::Map, score::Score},
    states::{
        gameplay::{reset_state, StateGame},
        ui::game_over_panel::StateGameOverPanel,
    },
};
use bevy::prelude::*;

pub struct RustydokuGameplay;

impl Plugin for RustydokuGameplay {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(StateGame::Restart),
            (
                Score::reset_score,
                Map::reset_tiles,
                HeaderUI::show,
                reset_state,
                respawn_figures,
            )
                .chain(),
        );

        app.add_systems(OnExit(StateGame::GameOver), set_hide_game_over_panel);

        app.add_systems(OnEnter(StateGameOverPanel::Hidden), show_figures);

        app.add_systems(
            OnEnter(StateGame::GameOver),
            (clear_figures, hide_figures).chain(),
        );
    }
}
