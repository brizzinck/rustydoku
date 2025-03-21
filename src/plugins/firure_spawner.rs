use crate::{
    components::figure::FigureComponent,
    events::figure_spawner::SpawnFigureEvent,
    resource::figure_spawner::FigureSpawnerResource,
    states::{figure::placeholder::PlaceholderAnimationState, gameplay::GameState},
};
use bevy::prelude::*;

/// Plugin for the figure spawner
pub struct RustydokuFigureSpawnerPlugin;

impl Plugin for RustydokuFigureSpawnerPlugin {
    fn build(&self, app: &mut App) {
        debug!("Building RustydokuFigureSpawnerPlugin");

        trace!("Adding event SpawnFigure");
        app.add_event::<SpawnFigureEvent>();

        trace!("Adding systems when generating world");
        app.add_systems(
            OnEnter(GameState::GenerateWorld),
            FigureSpawnerResource::spawn_zone_figures,
        );

        trace!("Adding systems just updating");
        app.add_systems(Update, FigureSpawnerResource::spawn_figures);

        app.add_systems(
            Update,
            (
                FigureSpawnerResource::adding_upscaling_figures,
                FigureSpawnerResource::lerping_figures,
                FigureSpawnerResource::upscaling_figures,
            )
                .chain(),
        );

        trace!("Adding systems when placing figures");
        app.add_systems(
            Update,
            (
                FigureSpawnerResource::removig_lerp_figures,
                FigureComponent::placing,
                FigureSpawnerResource::adding_lerp_figures,
            )
                .run_if(GameState::when_placing)
                .chain(),
        );

        trace!("Adding systems when placed figures");
        app.add_systems(
            Update,
            FigureSpawnerResource::despawn_figure.run_if(GameState::when_placed),
        );

        trace!("Inserting state StatePlaceholderAnimation");
        app.insert_state(PlaceholderAnimationState::default());

        debug!("RustydokuFigureSpawnerPlugin built");
    }
}
