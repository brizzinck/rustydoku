use crate::{
    components::figure::Figure,
    constants::{animation::ELAPSED_SCALE, figure::*},
    states::figure::StateFigureAnimation,
};
use bevy::prelude::*;

impl Figure {
    pub(crate) fn upscaling_when_drag(
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

        if transform.scale.x >= FIGURE_DRAGGING_SCALE - ELAPSED_SCALE {
            transform.scale = Vec3::splat(FIGURE_DRAGGING_SCALE);
            *state = StateFigureAnimation::default();
        } else {
            *state = StateFigureAnimation::DragUpScaling;
        }
    }
}
