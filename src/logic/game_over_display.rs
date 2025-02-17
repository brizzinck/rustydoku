use crate::{
    components::ui::{game_over::GameOverPanel, header::HeaderUI},
    constants::{
        game_over::{camera::*, ui_panel::GAME_OVER_PANEL_TOP_END_FLOAT},
        idle::camera::*,
        ui::game_over_panel::*,
    },
    states::StateGame,
};
use bevy::prelude::*;

pub fn display(
    mut camera: Query<(&Transform, &mut OrthographicProjection), With<Camera2d>>,
    state: Res<State<StateGame>>,
    time: Res<Time>,
) {
    if *state.get() != StateGame::GameOver {
        for (_transform, mut orthographic_projection) in camera.iter_mut() {
            orthographic_projection.viewport_origin.y =
                orthographic_projection.viewport_origin.y.lerp(
                    CAMERA_POSITION_Y_IDLE,
                    time.delta_secs() * CAMERA_ANIMATION_IN_POSITION_SPEED,
                );
        }
        return;
    }

    for (_transform, mut orthographic_projection) in camera.iter_mut() {
        orthographic_projection.viewport_origin.y = orthographic_projection.viewport_origin.y.lerp(
            CAMERA_POSITION_Y_GAME_OVER,
            time.delta_secs() * CAMERA_ANIMATION_OUT_POSITION_SPEED,
        );
    }
}

pub fn animate_game_over_panel(
    time: Res<Time>,
    mut query: Query<(&mut Node, &mut GameOverPanel)>,
    current_state: Res<State<StateGame>>,
) {
    if *current_state.get() != StateGame::GameOver {
        return;
    }

    for (mut style, mut panel) in query.iter_mut() {
        panel.speed += time.delta_secs();

        let speed = panel.speed;
        panel.timer.tick(time.delta().mul_f32(speed));

        let progress = panel.timer.elapsed_secs() / panel.timer.duration().as_secs_f32();

        style.top = Val::Percent(
            GAME_OVER_PANEL_TOP_DEFAULT_FLOAT - GAME_OVER_PANEL_TOP_END_FLOAT * progress,
        );
    }
}

pub fn hide_header(
    mut query: Query<&mut Visibility, With<HeaderUI>>,
    current_state: Res<State<StateGame>>,
) {
    if *current_state.get() != StateGame::GameOver {
        return;
    }

    for mut style in query.iter_mut() {
        *style = Visibility::Hidden;
    }
}
