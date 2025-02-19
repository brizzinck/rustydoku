use crate::{
    components::ui::header::HeaderUI,
    logic::{
        animation::game_over_panel::set_hide_game_over_panel,
        gameplay::figure_spawner::{clear_figures, hide_figures, respawn_figures, show_figures},
    },
    resource::{map::Map, score::Score},
    states::gameplay::{reset_state, StateGame},
};
use bevy::prelude::*;

pub struct RustydokuGameplay;

impl Plugin for RustydokuGameplay {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(StateGame::Restart),
            (
                respawn_figures,
                Score::reset_score,
                Map::reset_tiles,
                set_hide_game_over_panel,
                HeaderUI::show,
                show_figures,
                reset_state,
            )
                .chain(),
        );

        app.add_systems(
            OnEnter(StateGame::GameOver),
            (clear_figures, hide_figures).chain(),
        );
    }
}
