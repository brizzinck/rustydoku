use crate::{
    components::world::camera::RustyCamera2DComponent,
    constants::{animation::ELAPSED_TIME, world::camera::*},
    states::world::camera::CameraPositionState,
};
use bevy::prelude::*;

impl RustyCamera2DComponent {
    /// Animates the camera's vertical position toward the game over target.
    ///
    /// This function smoothly moves the camera’s viewport origin along the y-axis toward the
    /// game over position, defined by [`CAMERA_POSITION_Y_GAME_OVER`]. The movement speed is controlled by
    /// the frame delta time and the [`CAMERA_ANIMATION_OUT_POSITION_SPEED`] constant. Once the target
    /// position is nearly reached (within [`ELAPSED_TIME`]), the state is set to [`CameraPositionState::GameOver`].
    ///
    /// # Parameters
    /// - `camera`: A mutable query accessing the camera’s [`OrthographicProjection`] for 2D cameras.
    /// - `time`: The time resource providing the delta time for frame-rate independent movement.
    /// - `next_state`: A mutable reference to the next camera position state to be set.
    pub(crate) fn setting_game_over(
        mut camera: Query<&mut OrthographicProjection, With<Camera2d>>,
        time: Res<Time>,
        mut next_state: ResMut<NextState<CameraPositionState>>,
    ) {
        trace!("Setting camera to game over position");
        if RustyCamera2DComponent::setting_position(
            &mut camera,
            CAMERA_POSITION_Y_GAME_OVER,
            time.delta_secs(),
            CAMERA_ANIMATION_OUT_POSITION_SPEED,
        ) {
            trace!("Next state is game over position");
            next_state.set(CameraPositionState::GameOver);
        }
    }

    /// Animates the camera's vertical position back to the default idle position.
    ///
    /// This function smoothly moves the camera’s viewport origin along the y-axis toward the
    /// idle position defined by [`CAMERA_POSITION_Y_IDLE`]. The transition speed is determined by the
    /// frame delta time and the [`CAMERA_ANIMATION_IN_POSITION_SPEED`] constant. When the camera’s y-coordinate
    /// is close enough to the target (within [`ELAPSED_TIME`]), the state is set to [`CameraPositionState::Default`].
    ///
    /// # Parameters
    /// - `camera`: A mutable query accessing the camera’s [`OrthographicProjection`] for 2D cameras.
    /// - `time`: The time resource providing the delta time for smooth animation.
    /// - `next_state`: A mutable reference to the next camera position state.
    pub(crate) fn setting_default(
        mut camera: Query<&mut OrthographicProjection, With<Camera2d>>,
        time: Res<Time>,
        mut next_state: ResMut<NextState<CameraPositionState>>,
    ) {
        trace!("Setting camera to default position");
        if RustyCamera2DComponent::setting_position(
            &mut camera,
            CAMERA_POSITION_Y_IDLE,
            time.delta_secs(),
            CAMERA_ANIMATION_IN_POSITION_SPEED,
        ) {
            trace!("Next state is default position");
            next_state.set(CameraPositionState::Default);
        }
    }

    /// Interpolates the camera’s vertical position toward a target y-coordinate.
    ///
    /// This helper function adjusts the camera's [`OrthographicProjection::viewport_origin.y`]
    /// by linearly interpolating it toward the provided `to_y_position`. The interpolation is scaled by
    /// the product of the frame delta time (`delta`) and the specified `speed`.
    ///
    /// When the camera's y-coordinate is within [`ELAPSED_TIME`] of the target, it is snapped to the target
    /// position and the function returns `true` to indicate completion; otherwise, it returns `false`.
    ///
    /// # Parameters
    /// - `camera`: A mutable query accessing the camera’s [`OrthographicProjection`] component.
    /// - `to_y_position`: The target y-coordinate to move the camera toward.
    /// - `delta`: The time delta (in seconds) for the current frame.
    /// - `speed`: A scalar multiplier for the interpolation speed.
    ///
    /// # Returns
    /// - `true` if the camera's y-coordinate has reached (or is within [`ELAPSED_TIME`] of) the target.
    /// - `false` if the camera is still moving toward the target.
    fn setting_position(
        camera: &mut Query<&mut OrthographicProjection, With<Camera2d>>,
        to_y_position: f32,
        delta: f32,
        speed: f32,
    ) -> bool {
        let mut orthographic_projection = camera.single_mut();
        orthographic_projection.viewport_origin.y = orthographic_projection
            .viewport_origin
            .y
            .lerp(to_y_position, delta * speed);

        if (orthographic_projection.viewport_origin.y - to_y_position).abs() < ELAPSED_TIME {
            orthographic_projection.viewport_origin.y = to_y_position;
            true
        } else {
            false
        }
    }
}
