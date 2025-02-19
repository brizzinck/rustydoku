use crate::{
    components::figures::{square::Square, Figure},
    events::figure::FigureTriggerDragging,
    logic::{
        animation::figure::upscaling_dragging,
        figure::{dragging::moving, placing::placing, trigger::stop_dragging},
        gameplay::figure_spawner::{
            animation::{adding_lerp_figures, lerping_figures, removig_lerp_figures},
            despawn_figure,
            init::spawn_zone_figures,
            spawn_figures,
        },
    },
    resource::figure_spawner::FigureSpawner,
    states::gameplay::StateGame,
};
use bevy::prelude::*;

#[cfg(feature = "debug-inspector")]
use bevy_inspector_egui::prelude::*;

pub struct FigurePlugin;

impl Plugin for FigurePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(StateGame::GenerateWorld),
            (spawn_zone_figures, spawn_figures).chain(),
        );

        app.add_systems(Update, lerping_figures);

        app.add_systems(
            Update,
            (
                stop_dragging,
                moving,
                Square::highlight,
                call_dragging_events,
            )
                .run_if(StateGame::when_draggin)
                .chain(),
        );

        app.add_systems(
            Update,
            (removig_lerp_figures, placing, adding_lerp_figures)
                .run_if(StateGame::when_placing)
                .chain(),
        );

        app.add_systems(Update, despawn_figure.run_if(StateGame::when_placed));

        #[cfg(feature = "debug-inspector")]
        {
            use crate::components::figures::FigureBounds;
            app.register_type::<FigureBounds>();
        }
    }
}

fn call_dragging_events(
    mut event_reader: EventReader<FigureTriggerDragging>,
    mut figures: Query<(&mut Transform, &mut Figure)>,
    mut figure_spawner: ResMut<FigureSpawner>,
    time: Res<Time>,
) {
    for entity in event_reader.read() {
        let entity = entity.0;
        let (mut transform, mut figure) = figures.get_mut(entity).unwrap();

        upscaling_dragging(
            &mut transform,
            time.delta_secs(),
            &mut figure.state_animation,
        );

        figure_spawner.remove_lerp_figure(entity);
    }
}
