use crate::{
    components::figures::Figure,
    events::figure::FigureTriggerDragging,
    logic::{
        animation::figure::upscaling_dragging,
        figure::{dragging::moving, placing::placing, square::highlight, trigger::stop_dragging},
    },
    resource::figure_spawner::{
        adding_lerp_figures, despawn_figures, lerping_figures, removig_lerp_figures, spawn_figures,
        spawn_zone_figures, FigureSpawner,
    },
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
            (stop_dragging, moving, highlight, call_dragging_events)
                .run_if(StateGame::when_draggin)
                .chain(),
        );

        app.add_systems(
            Update,
            (removig_lerp_figures, placing, adding_lerp_figures)
                .run_if(StateGame::when_placing)
                .chain(),
        );

        app.add_systems(Update, despawn_figures.run_if(StateGame::when_placed));

        #[cfg(feature = "debug-inspector")]
        {
            use crate::components::figures::FigureBounds;
            app.register_type::<FigureBounds>();
        }
    }
}

fn call_dragging_events(
    mut event_reader: EventReader<FigureTriggerDragging>,
    mut figures: Query<&mut Transform, With<Figure>>,
    mut figure_spawner: ResMut<FigureSpawner>,
    time: Res<Time>,
) {
    for figure in event_reader.read() {
        let entity = figure.0;
        let mut transform = figures.get_mut(entity).unwrap();

        upscaling_dragging(&mut transform, time.delta_secs());

        figure_spawner.remove_lerp_figure(entity);
    }
}
