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
    /// Adds figure entities to the lerp queue when placement is denied.
    ///
    /// This function listens for [`FigureDeniedPlacingEvent`] events and adds the associated figure entity
    /// to the lerp queue so that it can be animated back to its placeholder position.
    ///
    /// # Parameters
    /// - `event_reader`: An event reader for [`FigureDeniedPlacingEvent`] events.
    /// - `figure_spawner`: A mutable reference to the figure spawner resource.
    pub(crate) fn adding_lerp_figures(
        mut event_reader: EventReader<FigureDeniedPlacingEvent>,
        mut figure_spawner: ResMut<FigureSpawnerResource>,
    ) {
        for FigureDeniedPlacingEvent(entity) in event_reader.read() {
            debug!("Adding lerp figure: {:?}", entity);
            figure_spawner.add_lerp_figure(*entity);
        }
    }

    /// Removes figure entities from the lerp queue when a trigger-up event occurs.
    ///
    /// This function listens for [`FigureTriggerUpEvent`] events and removes the corresponding figure entity
    /// from the lerp queue, indicating that the figure no longer needs to be animated back to its placeholder.
    ///
    /// # Parameters
    /// - `event_reader`: An event reader for [`FigureTriggerUpEvent`] events.
    /// - `figure_spawner`: A mutable reference to the figure spawner resource.
    pub(crate) fn removig_lerp_figures(
        mut event_reader: EventReader<FigureTriggerUpEvent>,
        mut figure_spawner: ResMut<FigureSpawnerResource>,
    ) {
        for FigureTriggerUpEvent(entity) in event_reader.read() {
            debug!("Removing lerp figure: {:?}", entity);
            figure_spawner.remove_lerp_figure(*entity);
        }
    }

    /// Adds figure entities to the upscaling queue upon spawning.
    ///
    /// This function listens for [`FigureSpawnedEvent`] events and adds the spawned figure entity
    /// to the upscaling queue so that it can be animated (scaled) during its spawn-up phase.
    ///
    /// # Parameters
    /// - `event_reader`: An event reader for [`FigureSpawnedEvent`] events.
    /// - `figure_spawner`: A mutable reference to the figure spawner resource.
    pub(crate) fn adding_upscaling_figures(
        mut event_reader: EventReader<FigureSpawnedEvent>,
        mut figure_spawner: ResMut<FigureSpawnerResource>,
    ) {
        for FigureSpawnedEvent(entity) in event_reader.read() {
            debug!("Adding upscaling figure: {:?}", entity);
            figure_spawner.add_upscaling_figure(*entity);
        }
    }

    /// Animates figures in the lerp queue towards their placeholder positions and scales.
    ///
    /// This function interpolates each figure's current position and scale toward its placeholder.
    /// When a figure is sufficiently close (determined by [`ELAPSED_SCALE`]), it snaps to the target,
    /// and its state is updated. Once complete, the figure is removed from the lerp queue.
    ///
    /// # Parameters
    /// - `figure_spawner`: A mutable reference to the figure spawner resource.
    /// - `figures`: `A query providing mutable access to both the [`FigureComponent`] and [`Transform`]
    ///    of each figure.`
    /// - `time`: The time resource, providing the delta seconds used for animation interpolation.
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

    /// Animates figures in the upscaling queue during their spawn-up phase.
    ///
    /// This function interpolates the figure's scale toward the idle scale.
    /// When the figure's scale is sufficiently close to the target (determined by [`ELAPSED_SCALE`]),
    /// its state is set to the default, and the figure is removed from the upscaling queue.
    ///
    /// # Parameters
    /// - `figure_spawner`: A mutable reference to the figure spawner resource.
    /// - `figures`: `A query providing mutable access to both the [`FigureComponent`] and [`Transform`]
    ///    of each figure.`
    /// - `time`: The time resource, used to determine the frame delta for smooth animation.
    pub(crate) fn upscaling_figures(
        mut figure_spawner: ResMut<FigureSpawnerResource>,
        mut figures: Query<(&mut FigureComponent, &mut Transform)>,
        time: Res<Time>,
    ) {
        let mut to_remove = Vec::new();

        for entity in figure_spawner.upscaling_figures.iter() {
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
            .upscaling_figures
            .retain(|entity| !to_remove.contains(entity));
    }
}
