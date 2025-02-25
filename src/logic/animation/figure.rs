use crate::{
    constants::figure::{
        FIGURE_DRAGGING_SCALE, FIGURE_SPEED_TO_UPSCALE,
        FIGURE_UPSCALE_SPEED_INCREMENT_FACTOR_PER_FRAME,
    },
    states::figure::StateFigureAnimation,
};
use bevy::prelude::*;

pub fn upscaling_dragging(transform: &mut Transform, delta: f32, state: &mut StateFigureAnimation) {
    transform.scale = transform.scale.lerp(
        Vec3::splat(FIGURE_DRAGGING_SCALE),
        delta
            * FIGURE_SPEED_TO_UPSCALE
            * (FIGURE_UPSCALE_SPEED_INCREMENT_FACTOR_PER_FRAME + transform.scale.x),
    );

    if transform.scale.x >= 0.99 {
        transform.scale = Vec3::splat(FIGURE_DRAGGING_SCALE);
        *state = StateFigureAnimation::default();
    } else {
        *state = StateFigureAnimation::UpScaling;
    }
}
