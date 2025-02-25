use crate::{
    components::ui::{game_over::GameOverPanel, header::HeaderUI},
    resource::{figure_spawner::FigureSpawner, map::Map, score::Score},
    states::{gameplay::StateGame, ui::game_over_panel::StateGameOverPanel},
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
                StateGame::reset_state,
                FigureSpawner::respawn_figures,
            )
                .chain(),
        );

        app.add_systems(OnExit(StateGame::GameOver), GameOverPanel::set_hide);

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
