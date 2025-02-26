use crate::{
    components::figures::Figure,
    constants::{
        animation::ELAPSED_SCALE,
        figure::{
            placeholder::*, FIGURE_IDEL_SCALE_VEC3, FIGURE_SPEED_TO_UPSCALING_SPAWN,
            FIGURE_UPSCALE_SPEED_INCREMENT_FACTOR_PER_FRAME,
        },
    },
    events::figure::{FigureDeniedPlacing, FigureSpawned, FigureTriggerUp},
    resource::figure_spawner::FigureSpawner,
    states::figure::StateFigureAnimation,
};
use bevy::prelude::*;
impl FigureSpawner {
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

    pub(crate) fn adding_upscaling_figures(
        mut event_reader: EventReader<FigureSpawned>,
        mut figure_spawner: ResMut<FigureSpawner>,
    ) {
        for FigureSpawned(entity) in event_reader.read() {
            figure_spawner.add_upscaling_figure(*entity);
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

                    if transform.translation.distance(*position) < ELAPSED_SCALE {
                        transform.translation = *position;
                    } else {
                        remove = false;
                    }
                }

                transform.scale = transform.scale.lerp(
                    LERPED_FIGURE_SCALE,
                    time.delta_secs() * FIGURE_SPEED_RETURN_TO_PLACEHOLDER,
                );

                if transform.scale.distance(LERPED_FIGURE_SCALE) < ELAPSED_SCALE {
                    transform.scale = LERPED_FIGURE_SCALE;
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

    pub(crate) fn upscaling_figures(
        mut figure_spawner: ResMut<FigureSpawner>,
        mut figures: Query<(&mut Figure, &mut Transform)>,
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
                    FIGURE_IDEL_SCALE_VEC3,
                    time.delta_secs()
                        * FIGURE_SPEED_TO_UPSCALING_SPAWN
                        * (FIGURE_UPSCALE_SPEED_INCREMENT_FACTOR_PER_FRAME + transform.scale.x),
                );

                if transform.scale.distance(FIGURE_IDEL_SCALE_VEC3) < ELAPSED_SCALE {
                    transform.scale = FIGURE_IDEL_SCALE_VEC3;
                    figure.state_animation = StateFigureAnimation::default();
                } else {
                    remove = false;
                    figure.state_animation = StateFigureAnimation::SpawnUpScaling;
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
