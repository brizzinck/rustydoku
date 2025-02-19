use crate::{
    components::{map::reset_tiles, ui::header::show_header},
    logic::{
        animation::game_over_panel::set_hide_game_over_panel,
        gameplay::figure_spawner::{clear_figures, hide_figures, respawn_figures, show_figures},
    },
    resource::score::Score,
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
                reset_tiles,
                set_hide_game_over_panel,
                show_header,
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
