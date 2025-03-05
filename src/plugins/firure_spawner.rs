use crate::{
    components::figure::Figure, resource::figure_spawner::FigureSpawner,
    states::gameplay::StateGame,
};
use bevy::prelude::*;

pub struct FigureSpawnerPlugin;

impl Plugin for FigureSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(StateGame::GenerateWorld),
            FigureSpawner::spawn_zone_figures,
        );

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

        app.add_systems(
            Update,
            FigureSpawner::despawn_figure.run_if(StateGame::when_placed),
        );
    }
}
