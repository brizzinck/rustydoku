use crate::{
    components::figures::Placeholder, constants::figure::placeholder::*,
    states::figure::placeholder::StatePlaceholderAnimation,
};
use bevy::prelude::*;

impl Placeholder {
    pub(crate) fn bouncing_init(
        mut placeholder_zones: Query<&mut Transform, With<Placeholder>>,
        mut next_state: ResMut<NextState<StatePlaceholderAnimation>>,
        time: Res<Time>,
    ) {
        let mut all_done = true;

        for mut transform in placeholder_zones.iter_mut() {
            let current_scale = transform.scale.x;
            if current_scale < PLACEHOLDER_SCALE_DEFAULT.x {
                all_done = false;

                transform.scale = Vec3::splat(
                    (current_scale
                        + time.delta_secs() * PLACEHOLDER_SCALE_SPEED_ANIMATION
                        + time.delta_secs() * PLACEHOLDER_SCALE_SPEED_UP_FACTOR * current_scale)
                        .min(PLACEHOLDER_SCALE_DEFAULT.x),
                );
            } else {
                transform.scale = PLACEHOLDER_SCALE_DEFAULT;
            }
        }

        if all_done {
            next_state.set(StatePlaceholderAnimation::BouncingDefault);
        }
    }

    pub(crate) fn bouncing_default(
        mut placeholder_zones: Query<&mut Transform, With<Placeholder>>,
        mut next_state: ResMut<NextState<StatePlaceholderAnimation>>,
        time: Res<Time>,
    ) {
        let mut all_done = true;

        for mut transform in placeholder_zones.iter_mut() {
            let current_scale = transform.scale.x;
            if current_scale < PLACEHOLDER_SCALE_PEAK {
                all_done = false;

                transform.scale = Vec3::splat(
                    (current_scale
                        + time.delta_secs() * PLACEHOLDER_SCALE_SPEED_ANIMATION
                        + time.delta_secs() * PLACEHOLDER_SCALE_SPEED_UP_FACTOR * current_scale)
                        .min(PLACEHOLDER_SCALE_PEAK),
                );
            } else {
                transform.scale = Vec3::splat(PLACEHOLDER_SCALE_PEAK);
            }
        }

        if all_done {
            next_state.set(StatePlaceholderAnimation::BouncingPeak);
        }
    }

    pub(crate) fn bouncing_peak(
        mut placeholder_zones: Query<&mut Transform, With<Placeholder>>,
        mut next_state: ResMut<NextState<StatePlaceholderAnimation>>,
        time: Res<Time>,
    ) {
        let mut all_done = true;

        for mut transform in placeholder_zones.iter_mut() {
            let current_scale = transform.scale.x;
            if current_scale > PLACEHOLDER_SCALE_DEFAULT.x {
                all_done = false;
                transform.scale = Vec3::splat(
                    (current_scale
                        - time.delta_secs() * PLACEHOLDER_SCALE_SPEED_ANIMATION
                        - time.delta_secs() * PLACEHOLDER_SCALE_SPEED_UP_FACTOR * current_scale)
                        .max(PLACEHOLDER_SCALE_DEFAULT.x),
                );
            } else {
                transform.scale = PLACEHOLDER_SCALE_DEFAULT;
            }
        }

        if all_done {
            next_state.set(StatePlaceholderAnimation::Idle);
        }
    }
}
