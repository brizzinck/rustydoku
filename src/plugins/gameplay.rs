use crate::{
    components::{
        ui::{game_over_panel::panel::GameOverPanelComponent, header::HeaderUI},
        world::placeholder::PlaceholderComponent,
    },
    resource::{figure_spawner::FigureSpawnerResource, map::MapComponent, score::ScoreResource},
    states::{gameplay::GameState, ui::game_over_panel::GameOverPanelState},
};
use bevy::{ecs::schedule::SystemConfigs, prelude::*};

/// Plugin for the gameplay logic
pub struct RustydokuGameplayPlugin;

impl RustydokuGameplayPlugin {
    fn general_restart() -> SystemConfigs {
        (
            MapComponent::reset_tiles,
            HeaderUI::show,
            GameState::reset_state,
        )
            .chain()
    }
}

impl Plugin for RustydokuGameplayPlugin {
    fn build(&self, app: &mut App) {
        debug!("Building RustydokuGameplayPlugin");

        trace!("Adding systems when restarting the game");
        app.add_systems(
            OnEnter(GameState::DefaultRestart),
            (
                Self::general_restart(),
                (
                    ScoreResource::reset_score,
                    PlaceholderComponent::set_bounce_default,
                    FigureSpawnerResource::respawn_figures,
                    PlaceholderComponent::reset_image,
                )
                    .chain(),
            ),
        );

        trace!("Adding systems when entering the game over state");
        app.add_systems(
            OnEnter(GameState::GameOver),
            (
                FigureSpawnerResource::clear_figures,
                FigureSpawnerResource::hide_figures,
            )
                .chain(),
        );

        trace!("Adding systems when entering the game over restart state");
        app.add_systems(OnEnter(GameState::GameOverRestart), Self::general_restart());

        trace!("Adding systems when exiting the game over state");
        app.add_systems(
            OnExit(GameState::GameOver),
            (ScoreResource::reset_score, GameOverPanelComponent::set_hide).chain(),
        );

        trace!("Adding systems when exiting the game over hidding state");
        app.add_systems(
            OnExit(GameOverPanelState::Hidding),
            (
                PlaceholderComponent::set_bounce_default,
                FigureSpawnerResource::show_figures,
                FigureSpawnerResource::respawn_figures,
                PlaceholderComponent::reset_image,
            ),
        );

        debug!("RustydokuGameplayPlugin build");
    }
}
