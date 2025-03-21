use crate::{
    components::world::placeholder::Placeholder, constants::placeholder::*,
    states::figure::placeholder::StatePlaceholderAnimation,
};
use bevy::prelude::*;

impl Placeholder {
    pub(crate) fn set_bounce_default(mut next_state: ResMut<NextState<StatePlaceholderAnimation>>) {
        next_state.set(StatePlaceholderAnimation::BouncingDefault);
        trace!("Placeholder animation set to BouncingDefault");
    }

    pub(crate) fn bouncing_init(
        mut placeholder_zones: Query<&mut Transform, With<Placeholder>>,
        mut next_state: ResMut<NextState<StatePlaceholderAnimation>>,
        time: Res<Time>,
    ) {
        let mut all_done = true;

        for mut transform in placeholder_zones.iter_mut() {
            Self::bouncing_init_logic(time.delta_secs(), &mut all_done, &mut transform);
        }

        if all_done {
            next_state.set(StatePlaceholderAnimation::BouncingDefault);
            trace!("Placeholder animation set to BouncingDefault");
        }
    }

    pub(crate) fn bouncing_default(
        mut placeholder_zones: Query<&mut Transform, With<Placeholder>>,
        mut next_state: ResMut<NextState<StatePlaceholderAnimation>>,
        time: Res<Time>,
    ) {
        let mut all_done = true;

        for mut transform in placeholder_zones.iter_mut() {
            Self::bouncing_default_logic(time.delta_secs(), &mut all_done, &mut transform);
        }

        if all_done {
            next_state.set(StatePlaceholderAnimation::BouncingPeak);
            trace!("Placeholder animation set to BouncingPeak");
        }
    }

    pub(crate) fn bouncing_peak(
        mut placeholder_zones: Query<&mut Transform, With<Placeholder>>,
        mut next_state: ResMut<NextState<StatePlaceholderAnimation>>,
        time: Res<Time>,
    ) {
        let mut all_done = true;

        for mut transform in placeholder_zones.iter_mut() {
            Self::bouncing_peak_logic(time.delta_secs(), &mut all_done, &mut transform);
        }

        if all_done {
            next_state.set(StatePlaceholderAnimation::Idle);
            trace!("Placeholder animation set to Idle");
        }
    }

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

        Placeholder::bouncing_peak_logic(delta, &mut all_done, &mut transform);

        let expected_scale = 1.31;

        assert_eq!(transform.scale, Vec3::splat(expected_scale));
    }

    #[test]
    fn bouncing_default_logic_works() {
        let delta = 0.1;

        let current_scale = PLACEHOLDER_SCALE_PEAK - 0.5;
        let mut transform = Transform::from_scale(Vec3::splat(current_scale));

        let mut all_done = true;

        Placeholder::bouncing_default_logic(delta, &mut all_done, &mut transform);

        let expected_scale = 0.7748;

        assert_eq!(transform.scale, Vec3::splat(expected_scale));
    }

    #[test]
    fn bouncing_init_logic_works() {
        let delta = 0.1;

        let current_scale = PLACEHOLDER_SCALE_DEFAULT.x - 0.5;
        let mut transform = Transform::from_scale(Vec3::splat(current_scale));

        let mut all_done = true;

        Placeholder::bouncing_init_logic(delta, &mut all_done, &mut transform);

        let expected_scale = 0.65;

        assert_eq!(transform.scale, Vec3::splat(expected_scale));
    }
}
