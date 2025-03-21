use crate::{
    components::world::camera::RustyCamera2DComponent,
    constants::{animation::ELAPSED_TIME, world::camera::*},
    states::world::camera::CameraPositionState,
};
use bevy::prelude::*;

impl RustyCamera2DComponent {
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
