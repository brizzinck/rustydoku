use bevy::prelude::*;

use crate::{
    components::ui::{game_over::GameOverPanel, header::HeaderUI},
    states::StateGame,
};

pub fn display(
    mut camera: Query<(&Transform, &mut OrthographicProjection), With<Camera2d>>,
    state: Res<State<StateGame>>,
    time: Res<Time>,
) {
    if *state.get() != StateGame::GameOver {
        for (_transform, mut orthographic_projection) in camera.iter_mut() {
            orthographic_projection.viewport_origin.y = orthographic_projection
                .viewport_origin
                .y
                .lerp(0.5, time.delta_secs() * 2.0);
        }
        return;
    }

    for (_transform, mut orthographic_projection) in camera.iter_mut() {
        orthographic_projection.viewport_origin.y = orthographic_projection
            .viewport_origin
            .y
            .lerp(0.65, time.delta_secs() * 2.0);
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

        style.top = Val::Percent(120.0 - 54.0 * progress);
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
