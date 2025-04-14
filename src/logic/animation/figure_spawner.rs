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
                if Self::lerping_figure_logic(
                    &figure_spawner,
                    &mut figure,
                    &mut transform,
                    entity,
                    time.delta_secs(),
                ) {
                    to_remove.push(*entity);
                }
            }
        }

        figure_spawner
            .lerp_figures
            .retain(|entity| !to_remove.contains(entity));
    }

    /// Animates a figure’s translation and scale toward its target values.
    ///
    /// This function performs two separate linear interpolations:
    ///
    /// 1. **Translation Lerp:** It retrieves the target position for the given entity
    ///    from the spawner’s `figures` map and lerps the current translation towards it.
    ///    If the distance between the new translation and the target is below `ELAPSED_SCALE`,
    ///    it snaps the translation exactly to the target.
    ///
    /// 2. **Scale Lerp:** Independently, it lerps the current scale toward a predefined target scale
    ///    (`FIGURE_SCALE_LERPED`). Similarly, if the scale is close enough (within `ELAPSED_SCALE`),
    ///    it snaps to the target and resets the figure’s animation state to default.
    ///    Otherwise, it sets the state to `BackLerping` to indicate the animation is ongoing.
    ///
    /// # Parameters
    /// - `spawner`: Reference to the figure spawner containing the target positions.
    /// - `figure`: Mutable reference to the figure component to update its animation state.
    /// - `transform`: Mutable reference to the figure’s transform (position and scale).
    /// - `entity`: The entity ID of the figure.
    /// - `delta`: The time delta (in seconds) since the last update.
    ///
    /// # Returns
    /// - `true` if both translation and scale have reached their targets and the figure
    ///   should be removed from the lerp queue; `false` otherwise.
    fn lerping_figure_logic(
        spawner: &FigureSpawnerResource,
        figure: &mut FigureComponent,
        transform: &mut Transform,
        entity: &Entity,
        delta: f32,
    ) -> bool {
        let mut remove = true;
        if let Some(position) = spawner.figures.get(entity) {
            transform.translation = transform
                .translation
                .lerp(*position, delta * FIGURE_RETURN_SPEED_TO_PLACEHOLDER);

            if transform.translation.distance(*position) < ELAPSED_SCALE {
                transform.translation = *position;
            } else {
                remove = false;
            }
        }

        transform.scale = transform.scale.lerp(
            FIGURE_SCALE_LERPED,
            delta * FIGURE_RETURN_SPEED_TO_PLACEHOLDER,
        );

        if transform.scale.distance(FIGURE_SCALE_LERPED) < ELAPSED_SCALE {
            transform.scale = FIGURE_SCALE_LERPED;
            figure.state_animation = FigureAnimationState::default();
        } else {
            figure.state_animation = FigureAnimationState::BackLerping;
            remove = false;
        }

        remove
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

                if Self::upscaling_figure_logic(&mut figure, &mut transform, time.delta_secs()) {
                    to_remove.push(*entity);
                }
            }
        }

        figure_spawner
            .upscaling_figures
            .retain(|entity| !to_remove.contains(entity));
    }

    /// Animates a figure’s scale during its spawn-up phase.
    ///
    /// This function linearly interpolates the figure's scale toward the idle scale (`FIGURE_IDLE_SCALE_VEC3`).
    /// It uses a dynamic interpolation factor that depends on the elapsed time (`delta`),
    /// a base speed, and the figure's current scale. If the new scale is within `ELAPSED_SCALE`
    /// of the target, the scale is snapped to the idle scale and the animation state is reset.
    ///
    /// # Parameters
    /// - `figure`: Mutable reference to the figure component to update its animation state.
    /// - `transform`: Mutable reference to the figure’s transform to update its scale.
    /// - `delta`: The time delta (in seconds) since the last update.
    ///
    /// # Returns
    /// - `true` if the figure's scale has reached the idle scale (animation complete); `false` otherwise.
    fn upscaling_figure_logic(
        figure: &mut FigureComponent,
        transform: &mut Transform,
        delta: f32,
    ) -> bool {
        transform.scale = transform.scale.lerp(
            FIGURE_IDLE_SCALE_VEC3,
            delta
                * FIGURE_SPAWN_UPSCALE_SPEED
                * (FIGURE_UPSCALE_SPEED_INCREMENT_PER_FRAME + transform.scale.x),
        );

        if transform.scale.distance(FIGURE_IDLE_SCALE_VEC3) < ELAPSED_SCALE {
            transform.scale = FIGURE_IDLE_SCALE_VEC3;
            figure.state_animation = FigureAnimationState::default();
            true
        } else {
            figure.state_animation = FigureAnimationState::SpawnUpScaling;
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_entity(id: u32) -> Entity {
        Entity::from_raw(id)
    }

    impl Default for FigureComponent {
        fn default() -> Self {
            Self {
                squares_entity: Default::default(),
                squares_position: Default::default(),
                state_animation: Default::default(),
                placeholder: dummy_entity(1),
            }
        }
    }
    #[test]
    fn lerping_figure_logic_works() {
        let entity = dummy_entity(2);
        let target_position = Vec3::splat(20.);
        let mut spawner = FigureSpawnerResource::default();
        spawner.figures.insert(entity, target_position);

        let mut figure = FigureComponent::default();
        figure.state_animation = FigureAnimationState::SpawnUpScaling;
        let mut transform = Transform::from_translation(Vec3::new(10., 10., 10.));

        let delta = 0.09;

        let completed = FigureSpawnerResource::lerping_figure_logic(
            &spawner,
            &mut figure,
            &mut transform,
            &entity,
            delta,
        );

        let expected_completed = false;
        assert_eq!(completed, expected_completed);

        let expected_translation = Vec3::splat(17.2);
        assert_eq!(transform.translation, expected_translation);

        let expected_scale = FigureAnimationState::BackLerping;
        assert_eq!(figure.state_animation, expected_scale);
    }

    #[test]
    fn upscaling_figure_logic_works() {
        let mut figure = FigureComponent::default();
        figure.state_animation = FigureAnimationState::SpawnUpScaling;
        let mut transform = Transform {
            scale: FIGURE_IDLE_SCALE_VEC3 + Vec3::splat(1.),
            ..Default::default()
        };

        let delta = 0.08;

        let completed =
            FigureSpawnerResource::upscaling_figure_logic(&mut figure, &mut transform, delta);

        let expected_completed = false;
        assert_eq!(completed, expected_completed);

        let expected_scale = Vec3::splat(0.5600002);
        assert_eq!(transform.scale, expected_scale);

        let expected_state = FigureAnimationState::SpawnUpScaling;
        assert_eq!(figure.state_animation, expected_state);
    }
}
