use crate::{
    components::ui::{game_over_panel::panel::GameOverPanel, header::HeaderUI},
    resource::{figure_spawner::FigureSpawner, map::Map, score::Score},
    states::{gameplay::StateGame, ui::game_over_panel::StateGameOverPanel},
};
use bevy::{ecs::schedule::SystemConfigs, prelude::*};

pub struct RustydokuGameplay;

impl Plugin for RustydokuGameplay {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(StateGame::DefaultRestart),
            (general_restart(), Score::reset_score).chain(),
        );

        app.add_systems(OnEnter(StateGame::GameOverRestart), general_restart());

        app.add_systems(
            OnExit(StateGame::GameOver),
            (Score::reset_score, GameOverPanel::set_hide).chain(),
        );

        app.add_systems(
            OnEnter(StateGameOverPanel::Hidden),
            FigureSpawner::show_figures,
        );

        app.add_systems(
            OnEnter(StateGame::GameOver),
            (FigureSpawner::clear_figures, FigureSpawner::hide_figures).chain(),
        );
    }
}

fn general_restart() -> SystemConfigs {
    (
        Map::reset_tiles,
        HeaderUI::show,
        StateGame::reset_state,
        FigureSpawner::respawn_figures,
    )
        .chain()
}
