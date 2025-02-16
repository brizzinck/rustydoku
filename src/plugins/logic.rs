use crate::{
    components::ui::game_over::spawn_game_over_panel,
    logic::{
        check_combo::check_combination,
        check_game_over::check_game_over,
        game_over_display::{animate_game_over_panel, display, hide_header},
        restart::restart,
    },
    resource::{
        figure_spawner::{clear_figures, hidden_figures},
        score::update_max_score,
    },
    states::StateGame,
};
use bevy::prelude::*;

pub struct RustydokuLogicPlugin;

impl Plugin for RustydokuLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(StateGame::CheckCombo), check_combination);
        app.add_systems(OnEnter(StateGame::CheckGameOver), check_game_over);
        app.add_systems(OnEnter(StateGame::Restart), restart);

        app.add_systems(
            OnEnter(StateGame::GameOver),
            (
                update_max_score,
                clear_figures,
                hidden_figures,
                spawn_game_over_panel,
            )
                .chain(),
        );
        app.add_systems(Update, (display, animate_game_over_panel, hide_header));
    }
}
