use crate::{
    components::world::placeholder::PlaceholderComponent, constants::placeholder::*,
    states::figure::placeholder::PlaceholderAnimationState,
};
use bevy::prelude::*;

impl PlaceholderComponent {
    /// Sets the placeholder animation state to [`BouncingDefault`].
    ///
    /// This is a helper method to transition directly to the default bouncing phase.
    pub(crate) fn set_bounce_default(mut next_state: ResMut<NextState<PlaceholderAnimationState>>) {
        next_state.set(PlaceholderAnimationState::BouncingDefault);
        trace!("Placeholder animation set to BouncingDefault");
    }

    /// Executes the initial bounce animation for all placeholders.
    ///
    /// Smoothly scales each placeholder from `0.0` to the default scale.
    /// Once all animations are complete, transitions to [`BouncingDefault`] state.
    pub(crate) fn bouncing_init(
        mut placeholder_zones: Query<&mut Transform, With<PlaceholderComponent>>,
        mut next_state: ResMut<NextState<PlaceholderAnimationState>>,
        time: Res<Time>,
    ) {
        let mut all_done = true;

        for mut transform in placeholder_zones.iter_mut() {
            Self::bouncing_init_logic(time.delta_secs(), &mut all_done, &mut transform);
        }

        if all_done {
            next_state.set(PlaceholderAnimationState::BouncingDefault);
            trace!("Placeholder animation set to BouncingDefault");
        }
    }

    /// Executes the bounce-up phase of the animation for all placeholders.
    ///
    /// Scales each placeholder up to a peak value for a "pop" effect.
    /// Once all have reached the peak, transitions to [`BouncingPeak`] state.
    pub(crate) fn bouncing_default(
        mut placeholder_zones: Query<&mut Transform, With<PlaceholderComponent>>,
        mut next_state: ResMut<NextState<PlaceholderAnimationState>>,
        time: Res<Time>,
    ) {
        let mut all_done = true;

        for mut transform in placeholder_zones.iter_mut() {
            Self::bouncing_default_logic(time.delta_secs(), &mut all_done, &mut transform);
        }

        if all_done {
            next_state.set(PlaceholderAnimationState::BouncingPeak);
            trace!("Placeholder animation set to BouncingPeak");
        }
    }

    /// Executes the bounce-down phase of the animation for all placeholders.
    ///
    /// Returns each placeholder’s scale from peak back to default.
    /// Once all placeholders are restored, transitions to [`Idle`] state.
    pub(crate) fn bouncing_peak(
        mut placeholder_zones: Query<&mut Transform, With<PlaceholderComponent>>,
        mut next_state: ResMut<NextState<PlaceholderAnimationState>>,
        time: Res<Time>,
    ) {
        let mut all_done = true;

        for mut transform in placeholder_zones.iter_mut() {
            Self::bouncing_peak_logic(time.delta_secs(), &mut all_done, &mut transform);
        }

        if all_done {
            next_state.set(PlaceholderAnimationState::Idle);
            trace!("Placeholder animation set to Idle");
        }
    }

    /// Core logic for the `bouncing_default` phase (scale up to peak).
    fn bouncing_default_logic(delta: f32, all_done: &mut bool, transform: &mut Transform) {
        let current_scale = transform.scale.x;
        if current_scale < PLACEHOLDER_SCALE_PEAK {
            *all_done = false;

            transform.scale = Vec3::splat(
                (current_scale
                    + delta * PLACEHOLDER_ANIMATION_SPEED
                    + delta * PLACEHOLDER_SCALE_UP_FACTOR * current_scale)
                    .min(PLACEHOLDER_SCALE_PEAK),
            );
        } else {
            transform.scale = Vec3::splat(PLACEHOLDER_SCALE_PEAK);
        }
    }

    /// Core logic for the `bouncing_peak` phase (scale down to default).
    fn bouncing_peak_logic(delta: f32, all_done: &mut bool, transform: &mut Transform) {
        let current_scale = transform.scale.x;
        if current_scale > PLACEHOLDER_SCALE_DEFAULT.x {
            *all_done = false;
            transform.scale = Vec3::splat(
                (current_scale
                    - delta * PLACEHOLDER_ANIMATION_SPEED
                    - delta * PLACEHOLDER_SCALE_UP_FACTOR * current_scale)
                    .max(PLACEHOLDER_SCALE_DEFAULT.x),
            );
        } else {
            transform.scale = PLACEHOLDER_SCALE_DEFAULT;
        }
    }

    /// Core logic for the `bouncing_init` phase (scale up from 0 to default).
    fn bouncing_init_logic(delta: f32, all_done: &mut bool, transform: &mut Transform) {
        let current_scale = transform.scale.x;
        if current_scale < PLACEHOLDER_SCALE_DEFAULT.x {
            *all_done = false;

            transform.scale = Vec3::splat(
                (current_scale
                    + delta * PLACEHOLDER_ANIMATION_SPEED
                    + delta * PLACEHOLDER_SCALE_UP_FACTOR * current_scale)
                    .min(PLACEHOLDER_SCALE_DEFAULT.x),
            );
        } else {
            transform.scale = PLACEHOLDER_SCALE_DEFAULT;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bouncing_peak_logic_works() {
        let delta = 0.1;

        let current_scale = PLACEHOLDER_SCALE_DEFAULT.x + 0.5;
        let mut transform = Transform::from_scale(Vec3::splat(current_scale));

        let mut all_done = true;

        PlaceholderComponent::bouncing_peak_logic(delta, &mut all_done, &mut transform);

        let expected_scale = 1.31;

        assert_eq!(transform.scale, Vec3::splat(expected_scale));
    }

    #[test]
    fn bouncing_default_logic_works() {
        let delta = 0.1;

        let current_scale = PLACEHOLDER_SCALE_PEAK - 0.5;
        let mut transform = Transform::from_scale(Vec3::splat(current_scale));

        let mut all_done = true;

        PlaceholderComponent::bouncing_default_logic(delta, &mut all_done, &mut transform);

        let expected_scale = 0.7748;

        assert_eq!(transform.scale, Vec3::splat(expected_scale));
    }

    #[test]
    fn bouncing_init_logic_works() {
        let delta = 0.1;

        let current_scale = PLACEHOLDER_SCALE_DEFAULT.x - 0.5;
        let mut transform = Transform::from_scale(Vec3::splat(current_scale));

        let mut all_done = true;

        PlaceholderComponent::bouncing_init_logic(delta, &mut all_done, &mut transform);

        let expected_scale = 0.65;

        assert_eq!(transform.scale, Vec3::splat(expected_scale));
    }
}
