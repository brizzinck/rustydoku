use crate::{
    components::figure::FigureComponent,
    constants::{
        animation::ELAPSED_SCALE,
        figure::*,
        placeholder::{FIGURE_RETURN_SPEED_TO_PLACEHOLDER, FIGURE_SCALE_LERPED},
    },
    events::figure::{FigureDeniedPlacingEvent, FigureSpawnedEvent, FigureTriggerUpEvent},
    resource::figure_spawner::FigureSpawnerResource,
    states::figure::FigureAnimationState,
};
use bevy::prelude::*;
impl FigureSpawnerResource {
    pub(crate) fn adding_lerp_figures(
        mut event_reader: EventReader<FigureDeniedPlacingEvent>,
        mut figure_spawner: ResMut<FigureSpawnerResource>,
    ) {
        for FigureDeniedPlacingEvent(entity) in event_reader.read() {
            debug!("Adding lerp figure: {:?}", entity);
            figure_spawner.add_lerp_figure(*entity);
        }
    }

    pub(crate) fn removig_lerp_figures(
        mut event_reader: EventReader<FigureTriggerUpEvent>,
        mut figure_spawner: ResMut<FigureSpawnerResource>,
    ) {
        for FigureTriggerUpEvent(entity) in event_reader.read() {
            debug!("Removing lerp figure: {:?}", entity);
            figure_spawner.remove_lerp_figure(*entity);
        }
    }

    pub(crate) fn adding_upscaling_figures(
        mut event_reader: EventReader<FigureSpawnedEvent>,
        mut figure_spawner: ResMut<FigureSpawnerResource>,
    ) {
        for FigureSpawnedEvent(entity) in event_reader.read() {
            debug!("Adding upscaling figure: {:?}", entity);
            figure_spawner.add_upscaling_figure(*entity);
        }
    }

    pub(crate) fn lerping_figures(
        mut figure_spawner: ResMut<FigureSpawnerResource>,
        mut figures: Query<(&mut FigureComponent, &mut Transform)>,
        time: Res<Time>,
    ) {
        let mut to_remove = Vec::new();
        for entity in figure_spawner.lerp_figures.iter() {
            if let Ok((mut figure, mut transform)) = figures.get_mut(*entity) {
                let mut remove = true;
                if let Some(position) = figure_spawner.figures.get(entity) {
                    transform.translation = transform.translation.lerp(
                        *position,
                        time.delta_secs() * FIGURE_RETURN_SPEED_TO_PLACEHOLDER,
                    );

                    if transform.translation.distance(*position) < ELAPSED_SCALE {
                        transform.translation = *position;
                    } else {
                        remove = false;
                    }
                }

                transform.scale = transform.scale.lerp(
                    FIGURE_SCALE_LERPED,
                    time.delta_secs() * FIGURE_RETURN_SPEED_TO_PLACEHOLDER,
                );

                if transform.scale.distance(FIGURE_SCALE_LERPED) < ELAPSED_SCALE {
                    transform.scale = FIGURE_SCALE_LERPED;
                    figure.state_animation = FigureAnimationState::default();
                } else {
                    remove = false;
                    figure.state_animation = FigureAnimationState::BackLerping;
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

    pub(crate) fn upscaling_figures(
        mut figure_spawner: ResMut<FigureSpawnerResource>,
        mut figures: Query<(&mut FigureComponent, &mut Transform)>,
        time: Res<Time>,
    ) {
        let mut to_remove = Vec::new();

        for entity in figure_spawner.bounce_figures.iter() {
            if let Ok((mut figure, mut transform)) = figures.get_mut(*entity) {
                if !figure.state_animation.is_spawn_upscaling()
                    && !figure.state_animation.is_default()
                {
                    to_remove.push(*entity);
                    continue;
                }

                let mut remove = true;
                transform.scale = transform.scale.lerp(
                    FIGURE_IDLE_SCALE_VEC3,
                    time.delta_secs()
                        * FIGURE_SPAWN_UPSCALE_SPEED
                        * (FIGURE_UPSCALE_SPEED_INCREMENT_PER_FRAME + transform.scale.x),
                );

                if transform.scale.distance(FIGURE_IDLE_SCALE_VEC3) < ELAPSED_SCALE {
                    transform.scale = FIGURE_IDLE_SCALE_VEC3;
                    figure.state_animation = FigureAnimationState::default();
                } else {
                    remove = false;
                    figure.state_animation = FigureAnimationState::SpawnUpScaling;
                }

                if remove {
                    to_remove.push(*entity);
                }
            }
        }

        figure_spawner
            .bounce_figures
            .retain(|entity| !to_remove.contains(entity));
    }
}
