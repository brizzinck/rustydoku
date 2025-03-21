use crate::{
    components::{
        ui::{game_over_panel::panel::GameOverPanel, header::HeaderUI},
        world::placeholder::Placeholder,
    },
    resource::{figure_spawner::FigureSpawner, map::Map, score::Score},
    states::{gameplay::StateGame, ui::game_over_panel::StateGameOverPanel},
};
use bevy::{ecs::schedule::SystemConfigs, prelude::*};

pub struct RustydokuGameplayPlugin;

impl RustydokuGameplayPlugin {
    fn general_restart() -> SystemConfigs {
        (Map::reset_tiles, HeaderUI::show, StateGame::reset_state).chain()
    }
}

impl Plugin for RustydokuGameplayPlugin {
    fn build(&self, app: &mut App) {
        debug!("Building RustydokuGameplayPlugin");

        trace!("Adding systems when restarting the game");
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

        trace!("Adding systems when entering the game over state");
        app.add_systems(
            OnEnter(StateGame::GameOver),
            (FigureSpawner::clear_figures, FigureSpawner::hide_figures).chain(),
        );

        trace!("Adding systems when entering the game over restart state");
        app.add_systems(OnEnter(StateGame::GameOverRestart), Self::general_restart());

        trace!("Adding systems when exiting the game over state");
        app.add_systems(
            OnExit(StateGame::GameOver),
            (Score::reset_score, GameOverPanel::set_hide).chain(),
        );

        trace!("Adding systems when exiting the game over hidding state");
        app.add_systems(
            OnExit(StateGameOverPanel::Hidding),
            (
                Placeholder::set_bounce_default,
                FigureSpawner::show_figures,
                FigureSpawner::respawn_figures,
                Placeholder::reset_image,
            ),
        );

        debug!("RustydokuGameplayPlugin build");
    }
}
