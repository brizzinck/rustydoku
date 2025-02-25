use crate::{
    components::figures::{square::Square, Figure, FigureZone, Placeholder},
    resource::{figure_spawner::FigureSpawner, square::SquaresToDespawn},
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
                FigureSpawner::spawn_figures,
            )
                .chain(),
        );

        app.add_systems(Update, FigureSpawner::lerping_figures);

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

        app.add_systems(Update, (call_squares_despawn, call_placeholder_animation));

        app.add_systems(
            Update,
            FigureSpawner::despawn_figure.run_if(StateGame::when_placed),
        );

        #[cfg(feature = "debug-inspector")]
        {
            use crate::components::figures::FigureBounds;
            app.register_type::<FigureBounds>();
        }
    }
}

fn call_squares_despawn(
    mut commands: Commands,
    mut squares_to_despawn: ResMut<SquaresToDespawn>,
    mut squares: Query<&mut Sprite, With<Square>>,
    time: Res<Time>,
) {
    squares_to_despawn.squares.retain(|entity| {
        if let Ok(mut sprite) = squares.get_mut(*entity) {
            if Square::fading_out(&mut sprite.color, time.delta_secs()) {
                commands.entity(*entity).despawn();
                false
            } else {
                true
            }
        } else {
            false
        }
    });
}

pub fn call_placeholder_animation(
    mut placeholder_zones: Query<&mut Transform, With<Placeholder>>,
    mut figure_zone: Query<&mut FigureZone>,
    time: Res<Time>,
) {
    let mut figure_zone = figure_zone.single_mut();
    let delta = time.delta_secs();
    let mut all_done = true;

    for entity in figure_zone.placeholders.clone().into_iter() {
        if let Ok(mut transform) = placeholder_zones.get_mut(entity) {
            let done = Placeholder::bouncing(
                &mut transform,
                delta,
                &figure_zone.placeholder_state_animation,
            );
            if !done {
                all_done = false;
            }
        }
    }

    if all_done {
        figure_zone.placeholder_state_animation = match figure_zone.placeholder_state_animation {
            StatePlaceholderAnimation::BouncingInit => StatePlaceholderAnimation::BouncingDefault,
            StatePlaceholderAnimation::BouncingDefault => StatePlaceholderAnimation::BouncingPeak,
            StatePlaceholderAnimation::BouncingPeak => StatePlaceholderAnimation::default(),
            ref other => other.clone(),
        };
    }
}
