use crate::{
    components::figures::Figure,
    constants::figure::placeholder::{FIGURE_SPEED_RETURN_TO_PLACEHOLDER, PLACEHOLDER_SCALE},
    events::figure::{FigureDeniedPlacing, FigureTriggerUp},
    resource::figure_spawner::FigureSpawner,
    states::figure::StateFigureAnimation,
};
use bevy::prelude::*;

pub(crate) fn adding_lerp_figures(
    mut event_reader: EventReader<FigureDeniedPlacing>,
    mut figure_spawner: ResMut<FigureSpawner>,
) {
    for FigureDeniedPlacing(entity) in event_reader.read() {
        figure_spawner.add_lerp_figure(*entity);
    }
}

pub(crate) fn removig_lerp_figures(
    mut event_reader: EventReader<FigureTriggerUp>,
    mut figure_spawner: ResMut<FigureSpawner>,
) {
    for FigureTriggerUp(entity) in event_reader.read() {
        figure_spawner.remove_lerp_figure(*entity);
    }
}

pub(crate) fn lerping_figures(
    mut figure_spawner: ResMut<FigureSpawner>,
    mut figures: Query<(&mut Figure, &mut Transform)>,
    time: Res<Time>,
) {
    let mut to_remove = Vec::new();
    for entity in figure_spawner.lerp_figures.iter() {
        if let Ok((mut figure, mut transform)) = figures.get_mut(*entity) {
            let mut remove = true;
            if let Some(position) = figure_spawner.figures.get(entity) {
                transform.translation = transform.translation.lerp(
                    *position,
                    time.delta_secs() * FIGURE_SPEED_RETURN_TO_PLACEHOLDER,
                );

                if transform.translation.distance(*position) < 0.01 {
                    transform.translation = *position;
                } else {
                    remove = false;
                }
            }

            transform.scale = transform.scale.lerp(
                Vec3::splat(PLACEHOLDER_SCALE),
                time.delta_secs() * FIGURE_SPEED_RETURN_TO_PLACEHOLDER,
            );

            if transform.scale.distance(Vec3::splat(PLACEHOLDER_SCALE)) < 0.01 {
                transform.scale = Vec3::splat(PLACEHOLDER_SCALE);
                figure.state_animation = StateFigureAnimation::default();
            } else {
                remove = false;
                figure.state_animation = StateFigureAnimation::BackLerping;
            }

            if remove {
                to_remove.push(*entity);
            }
        }
    }

    figure_spawner
        .lerp_figures
        .retain(|entity| !to_remove.contains(entity));
}
