use crate::{
    components::figure::FigureComponent,
    constants::{animation::ELAPSED_SCALE, figure::*},
    states::figure::FigureAnimationState,
};
use bevy::prelude::*;

impl FigureComponent {
    pub(crate) fn upscaling_when_drag(
        transform: &mut Transform,
        delta: f32,
        state: &mut FigureAnimationState,
    ) {
        transform.scale = transform.scale.lerp(
            Vec3::splat(FIGURE_DRAGGING_SCALE),
            delta
                * FIGURE_SPEED_UPSCALE
                * (FIGURE_UPSCALE_SPEED_INCREMENT_PER_FRAME + transform.scale.x),
        );

        if transform.scale.x >= FIGURE_DRAGGING_SCALE - ELAPSED_SCALE {
            transform.scale = Vec3::splat(FIGURE_DRAGGING_SCALE);
            *state = FigureAnimationState::default();
        } else {
            *state = FigureAnimationState::DragUpScaling;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upscaling_not_finished_works() {
        let initial_scale = FIGURE_IDLE_SCALE;
        let mut transform = Transform::from_scale(Vec3::splat(initial_scale));
        let mut state = FigureAnimationState::default();
        let delta = 0.016;

        FigureComponent::upscaling_when_drag(&mut transform, delta, &mut state);

        let expected_state = FigureAnimationState::DragUpScaling;
        assert_eq!(state, expected_state);

        let delte = delta
            * FIGURE_SPEED_UPSCALE
            * (FIGURE_UPSCALE_SPEED_INCREMENT_PER_FRAME + initial_scale);

        let expected_scale_value = initial_scale + (FIGURE_DRAGGING_SCALE - initial_scale) * delte;
        let expected_scale = Vec3::splat(expected_scale_value);

        assert_eq!(transform.scale, expected_scale);
    }

    #[test]
    fn upscaling_finished_works() {
        let mut transform =
            Transform::from_scale(Vec3::splat(FIGURE_DRAGGING_SCALE - (ELAPSED_SCALE / 2.0)));
        let mut state = FigureAnimationState::DragUpScaling;
        let delta = 1.0;

        FigureComponent::upscaling_when_drag(&mut transform, delta, &mut state);

        let expected_scale = Vec3::splat(FIGURE_DRAGGING_SCALE);
        assert_eq!(transform.scale, expected_scale);

        let expected_state = FigureAnimationState::Idle;
        assert_eq!(state, expected_state);
    }
}
