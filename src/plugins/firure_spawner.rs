use crate::{
    components::figure::Figure, resource::figure_spawner::FigureSpawner,
    states::gameplay::StateGame,
};
use bevy::prelude::*;

pub struct RustydokuFigureSpawnerPlugin;

impl Plugin for RustydokuFigureSpawnerPlugin {
    fn build(&self, app: &mut App) {
        debug!("Building RustydokuFigureSpawnerPlugin");

        trace!("Adding systems when generating world");
        app.add_systems(
            OnEnter(StateGame::GenerateWorld),
            FigureSpawner::spawn_zone_figures,
        );

        trace!("Adding systems just updating");
        app.add_systems(Update, FigureSpawner::spawn_figures);

        app.add_systems(
            Update,
            (
                FigureSpawner::adding_upscaling_figures,
                FigureSpawner::lerping_figures,
                FigureSpawner::upscaling_figures,
            )
                .chain(),
        );

        trace!("Adding systems when placing figures");
        app.add_systems(
            Update,
            (
                FigureSpawner::removig_lerp_figures,
                Figure::placing,
                FigureSpawner::adding_lerp_figures,
            )
                .run_if(StateGame::when_placing)
                .chain(),
        );

        trace!("Adding systems when placed figures");
        app.add_systems(
            Update,
            FigureSpawner::despawn_figure.run_if(StateGame::when_placed),
        );

        debug!("RustydokuFigureSpawnerPlugin built");
    }
}
