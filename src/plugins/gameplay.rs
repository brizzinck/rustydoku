use crate::{
    components::{
        ui::{game_over_panel::panel::GameOverPanel, header::HeaderUI},
        world::placeholder::Placeholder,
    },
    resource::{figure_spawner::FigureSpawner, map::Map, score::Score},
    states::{gameplay::StateGame, ui::game_over_panel::StateGameOverPanel},
};
use bevy::{ecs::schedule::SystemConfigs, prelude::*};

pub struct RustydokuGameplay;

impl RustydokuGameplay {
    fn general_restart() -> SystemConfigs {
        (Map::reset_tiles, HeaderUI::show, StateGame::reset_state).chain()
    }
}

impl Plugin for RustydokuGameplay {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(StateGame::DefaultRestart),
            (
                Self::general_restart(),
                (
                    Score::reset_score,
                    Placeholder::set_bounce_default,
                    FigureSpawner::respawn_figures,
                    Placeholder::reset_image,
                )
                    .chain(),
            ),
        );

        app.add_systems(
            OnEnter(StateGame::GameOver),
            (FigureSpawner::clear_figures, FigureSpawner::hide_figures).chain(),
        );

        app.add_systems(OnEnter(StateGame::GameOverRestart), Self::general_restart());

        app.add_systems(
            OnExit(StateGame::GameOver),
            (Score::reset_score, GameOverPanel::set_hide).chain(),
        );

        app.add_systems(
            OnExit(StateGameOverPanel::Hidding),
            (
                Placeholder::set_bounce_default,
                FigureSpawner::show_figures,
                FigureSpawner::respawn_figures,
                Placeholder::reset_image,
            ),
        );
    }
}
