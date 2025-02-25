use crate::{
    constants::figure::placeholder::*, states::figure::placeholder::StatePlaceholderAnimation,
};
use bevy::prelude::*;

pub fn bouncing(transform: &mut Transform, delta: f32, state: &StatePlaceholderAnimation) -> bool {
    let current_scale = transform.scale.x;
    match state {
        StatePlaceholderAnimation::BouncingInit => {
            if current_scale < PLACEHOLDER_SCALE_DEFAULT.x {
                transform.scale = Vec3::splat(
                    (current_scale
                        + delta * PLACEHOLDER_SCALE_SPEED_ANIMATION
                        + delta * PLACEHOLDER_SCALE_SPEED_UP_FACTOR * current_scale)
                        .min(PLACEHOLDER_SCALE_DEFAULT.x),
                );
            }
            transform.scale.x >= PLACEHOLDER_SCALE_DEFAULT.x
        }
        StatePlaceholderAnimation::BouncingDefault => {
            if current_scale < PLACEHOLDER_SCALE_PEAK {
                transform.scale = Vec3::splat(
                    (current_scale
                        + delta * PLACEHOLDER_SCALE_SPEED_ANIMATION
                        + delta * PLACEHOLDER_SCALE_SPEED_UP_FACTOR * current_scale)
                        .min(PLACEHOLDER_SCALE_PEAK),
                );
            }
            transform.scale.x >= PLACEHOLDER_SCALE_PEAK
        }
        StatePlaceholderAnimation::BouncingPeak => {
            if current_scale > PLACEHOLDER_SCALE_DEFAULT.x {
                transform.scale = Vec3::splat(
                    (current_scale
                        - delta * PLACEHOLDER_SCALE_SPEED_ANIMATION
                        - delta * PLACEHOLDER_SCALE_SPEED_UP_FACTOR * current_scale)
                        .max(PLACEHOLDER_SCALE_DEFAULT.x),
                );
            }
            transform.scale.x <= PLACEHOLDER_SCALE_DEFAULT.x
        }
        _ => true,
    }
}
