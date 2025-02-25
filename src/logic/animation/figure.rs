use crate::{
    components::figures::Figure,
    constants::{
        animation::ELAPSED_SCALE_TO_ONE,
        figure::{
            FIGURE_DRAGGING_SCALE, FIGURE_SPEED_TO_UPSCALE,
            FIGURE_UPSCALE_SPEED_INCREMENT_FACTOR_PER_FRAME,
        },
    },
    states::figure::StateFigureAnimation,
};
use bevy::prelude::*;

impl Figure {
    pub(crate) fn upscaling_when_dragging(
        transform: &mut Transform,
        delta: f32,
        state: &mut StateFigureAnimation,
    ) {
        transform.scale = transform.scale.lerp(
            Vec3::splat(FIGURE_DRAGGING_SCALE),
            delta
                * FIGURE_SPEED_TO_UPSCALE
                * (FIGURE_UPSCALE_SPEED_INCREMENT_FACTOR_PER_FRAME + transform.scale.x),
        );

        if transform.scale.x >= ELAPSED_SCALE_TO_ONE {
            transform.scale = Vec3::splat(FIGURE_DRAGGING_SCALE);
            *state = StateFigureAnimation::default();
        } else {
            *state = StateFigureAnimation::UpScaling;
        }
    }
}
