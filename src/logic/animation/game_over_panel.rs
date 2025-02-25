use crate::{
    components::ui::game_over::GameOverPanel, constants::ui::game_over_panel::*,
    states::ui::game_over_panel::StateGameOverPanel,
};
use bevy::prelude::*;

impl GameOverPanel {
    pub(crate) fn set_show(mut next_state: ResMut<NextState<StateGameOverPanel>>) {
        next_state.set(StateGameOverPanel::Showing);
    }

    pub(crate) fn set_hide(mut next_state: ResMut<NextState<StateGameOverPanel>>) {
        next_state.set(StateGameOverPanel::Hidding);
    }

    pub(crate) fn show(
        time: Res<Time>,
        mut query: Query<(&mut Node, &mut GameOverPanel)>,
        mut next_state: ResMut<NextState<StateGameOverPanel>>,
    ) {
        let (mut style, mut panel) = query.single_mut();

        if panel.timer.finished() {
            panel.timer.reset();
            panel.speed = GAME_OVER_PANEL_DEFAULT_SPEED_ANIMATION;
            next_state.set(StateGameOverPanel::Showed);
            return;
        }

        panel.speed += time.delta_secs() * GAME_OVER_PANEL_SHOW_MUL_SPEED_ANIMATION;

        let speed = panel.speed;
        panel.timer.tick(time.delta().mul_f32(speed));

        let progress = panel.timer.elapsed_secs() / panel.timer.duration().as_secs_f32();

        style.top = Val::Percent(
            GAME_OVER_PANEL_TOP_DEFAULT_FLOAT - GAME_OVER_PANEL_TOP_END_FLOAT * progress,
        );
    }

    pub(crate) fn hide(
        time: Res<Time>,
        mut query: Query<(&mut Node, &mut GameOverPanel)>,
        mut next_state: ResMut<NextState<StateGameOverPanel>>,
    ) {
        let (mut style, mut panel) = query.single_mut();

        if panel.timer.finished() {
            panel.timer.reset();
            panel.speed = GAME_OVER_PANEL_DEFAULT_SPEED_ANIMATION;
            next_state.set(StateGameOverPanel::Hidden);
            return;
        }

        panel.speed += time.delta_secs() * GAME_OVER_PANEL_HIDE_MUL_SPEED_ANIMATION;

        let speed = panel.speed;
        panel.timer.tick(time.delta().mul_f32(speed));

        let progress = panel.timer.elapsed_secs() / panel.timer.duration().as_secs_f32();

        style.top = Val::Percent(
            GAME_OVER_PANEL_TOP_REVERSE_END_FLOAT + GAME_OVER_PANEL_TOP_END_FLOAT * progress,
        );
    }
}
