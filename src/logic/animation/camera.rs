use crate::{
    constants::{animation::ELAPSED_TIME, game_over::camera::*, idle::camera::*},
    states::gameplay::game_over::StateCameraPosition,
};
use bevy::prelude::*;

pub fn setting_camera_game_over(
    mut camera: Query<(&Transform, &mut OrthographicProjection), With<Camera2d>>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<StateCameraPosition>>,
) {
    if setting_camera_position(
        &mut camera,
        CAMERA_POSITION_Y_GAME_OVER,
        time.delta_secs(),
        CAMERA_ANIMATION_OUT_POSITION_SPEED,
    ) {
        next_state.set(StateCameraPosition::GameOver);
    }
}

pub fn setting_camera_default(
    mut camera: Query<(&Transform, &mut OrthographicProjection), With<Camera2d>>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<StateCameraPosition>>,
) {
    if setting_camera_position(
        &mut camera,
        CAMERA_POSITION_Y_IDLE,
        time.delta_secs(),
        CAMERA_ANIMATION_IN_POSITION_SPEED,
    ) {
        next_state.set(StateCameraPosition::Default);
    }
}

fn setting_camera_position(
    camera: &mut Query<(&Transform, &mut OrthographicProjection), With<Camera2d>>,
    to_y_position: f32,
    delta: f32,
    speed: f32,
) -> bool {
    let (_transform, mut orthographic_projection) = camera.single_mut();
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
