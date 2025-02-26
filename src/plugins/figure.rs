use crate::{
    components::figures::{square::Square, Figure, Placeholder},
    resource::figure_spawner::FigureSpawner,
    states::{figure::placeholder::StatePlaceholderAnimation, gameplay::StateGame},
};
use bevy::prelude::*;

#[cfg(feature = "debug-inspector")]
use bevy_inspector_egui::prelude::*;

pub struct FigurePlugin;

impl Plugin for FigurePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(StateGame::GenerateWorld),
            (
                FigureSpawner::spawn_zone_figures,
                FigureSpawner::spawn_figures.after(FigureSpawner::spawn_zone_figures),
            ),
        );

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
                Figure::stop_dragging,
                Figure::moving,
                Square::highlight,
                Figure::call_dragging_events,
            )
                .run_if(StateGame::when_draggin)
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

        app.add_systems(Update, Square::call_despawn);

        app.add_systems(
            Update,
            FigureSpawner::despawn_figure.run_if(StateGame::when_placed),
        );

        app.add_systems(
            Update,
            Placeholder::bouncing_init.run_if(StatePlaceholderAnimation::when_bouncing_init),
        );

        app.add_systems(
            Update,
            Placeholder::bouncing_default.run_if(StatePlaceholderAnimation::when_bouncing_default),
        );

        app.add_systems(
            Update,
            Placeholder::bouncing_peak.run_if(StatePlaceholderAnimation::when_bouncing_peak),
        );

        #[cfg(feature = "debug-inspector")]
        {
            use crate::components::figures::FigureBounds;
            app.register_type::<FigureBounds>();
        }
    }
}
