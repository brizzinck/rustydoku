use crate::{
    components::camera::RustyCamera2D,
    constants::{animation::ELAPSED_TIME, world::camera::*},
    states::world::camera::StateCameraPosition,
};
use bevy::prelude::*;

impl RustyCamera2D {
    pub(crate) fn setting_game_over(
        mut camera: Query<&mut OrthographicProjection, With<Camera2d>>,
        time: Res<Time>,
        mut next_state: ResMut<NextState<StateCameraPosition>>,
    ) {
        if RustyCamera2D::setting_position(
            &mut camera,
            CAMERA_POSITION_Y_GAME_OVER,
            time.delta_secs(),
            CAMERA_ANIMATION_OUT_POSITION_SPEED,
        ) {
            next_state.set(StateCameraPosition::GameOver);
        }
    }

    pub(crate) fn setting_default(
        mut camera: Query<&mut OrthographicProjection, With<Camera2d>>,
        time: Res<Time>,
        mut next_state: ResMut<NextState<StateCameraPosition>>,
    ) {
        if RustyCamera2D::setting_position(
            &mut camera,
            CAMERA_POSITION_Y_IDLE,
            time.delta_secs(),
            CAMERA_ANIMATION_IN_POSITION_SPEED,
        ) {
            next_state.set(StateCameraPosition::Default);
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
