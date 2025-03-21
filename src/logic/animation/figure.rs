use crate::{
    components::figure::FigureComponent,
    constants::{animation::ELAPSED_SCALE, figure::*},
    states::figure::FigureAnimationState,
};
use bevy::prelude::*;

impl FigureComponent {
    /// Interpolates the figure's scale when it is being dragged.
    ///
    /// This function gradually scales the figure toward the target dragging scale,
    /// defined by [`FIGURE_DRAGGING_SCALE`]. The interpolation uses the frame delta time
    /// (`delta`) multiplied by a speed factor and an incremental adjustment based on the
    /// current scale.
    ///
    /// Once the figure's scale is sufficiently close (within [`ELAPSED_SCALE`]) to the target,
    /// the scale is snapped to [`FIGURE_DRAGGING_SCALE`] and the animation state is reset to
    /// the default state (commonly representing an idle state). Otherwise, the state is set to
    /// [`FigureAnimationState::DragUpScaling`] to indicate that the scaling animation is in progress.
    ///
    /// # Parameters
    /// - `transform`: Mutable reference to the figure's [`Transform`] component, which contains the current scale.
    /// - `delta`: Time delta (in seconds) for the current frame, used to ensure frame rate–independent animation.
    /// - `state`: Mutable reference to the figure's animation state, which will be updated based on progress.
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
