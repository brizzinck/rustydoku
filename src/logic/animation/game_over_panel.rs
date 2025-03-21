use crate::{
    components::ui::game_over_panel::panel::GameOverPanel, constants::ui::game_over_panel::*,
    states::ui::game_over_panel::StateGameOverPanel,
};
use bevy::prelude::*;

impl GameOverPanel {
    pub(crate) fn set_show(mut next_state: ResMut<NextState<StateGameOverPanel>>) {
        next_state.set(StateGameOverPanel::Showing);
        debug!("Next state set to StateGameOverPanel::Showing");
    }

    pub(crate) fn set_hide(mut next_state: ResMut<NextState<StateGameOverPanel>>) {
        next_state.set(StateGameOverPanel::Hidding);
        debug!("Next state set to StateGameOverPanel::Hidding");
    }

    pub(crate) fn show(
        time: Res<Time>,
        mut query: Query<(&mut Node, &mut GameOverPanel)>,
        mut next_state: ResMut<NextState<StateGameOverPanel>>,
    ) {
        trace!("Showing game over panel");

        let (mut style, mut panel) = query.single_mut();

        Self::show_logic(&mut style, &mut panel, &mut next_state, &time);
    }

    pub(crate) fn hide(
        time: Res<Time>,
        mut query: Query<(&mut Node, &mut GameOverPanel)>,
        mut next_state: ResMut<NextState<StateGameOverPanel>>,
    ) {
        trace!("Hidding game over panel");

        let (mut style, mut panel) = query.single_mut();

        Self::hide_logic(&mut style, &mut panel, &mut next_state, &time);
    }

    fn show_logic(
        style: &mut Node,
        panel: &mut GameOverPanel,
        next_state: &mut NextState<StateGameOverPanel>,
        time: &Time,
    ) {
        if panel.timer.finished() {
            panel.timer.reset();
            panel.speed = GAME_OVER_PANEL_ANIMATION_SPEED_DEFAULT;
            next_state.set(StateGameOverPanel::Showed);
            trace!("Game over panel showed");
            return;
        }

        panel.speed += time.delta_secs() * GAME_OVER_PANEL_ANIMATION_SPEED_SHOW_MULTIPLIER;

        let speed = panel.speed;
        panel.timer.tick(time.delta().mul_f32(speed));

        let progress = panel.timer.elapsed_secs() / panel.timer.duration().as_secs_f32();

        style.top =
            Val::Percent(GAME_OVER_PANEL_TOP_DEFAULT_VALUE - GAME_OVER_PANEL_TOP_END * progress);

        trace!(
            "Game over panel top: {:?}, progress: {:?}, speed: {:?}",
            style.top,
            progress,
            speed
        );
    }

    fn hide_logic(
        style: &mut Node,
        panel: &mut GameOverPanel,
        next_state: &mut NextState<StateGameOverPanel>,
        time: &Time,
    ) {
        if panel.timer.finished() {
            panel.timer.reset();
            panel.speed = GAME_OVER_PANEL_ANIMATION_SPEED_DEFAULT;
            next_state.set(StateGameOverPanel::Hidden);
            trace!("Game over panel hidden");
            return;
        }

        panel.speed += time.delta_secs() * GAME_OVER_PANEL_ANIMATION_SPEED_HIDE_MULTIPLIER;

        let speed = panel.speed;
        panel.timer.tick(time.delta().mul_f32(speed));

        let progress = panel.timer.elapsed_secs() / panel.timer.duration().as_secs_f32();

        style.top =
            Val::Percent(GAME_OVER_PANEL_TOP_END_REVERSED + GAME_OVER_PANEL_TOP_END * progress);

        trace!(
            "Game over panel top: {:?}, progress: {:?}, speed: {:?}",
            style.top,
            progress,
            speed
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn show_logic_works() {
        let time = Time::default();

        let mut next_state = NextState::default();
        next_state.set(StateGameOverPanel::Showing);

        let mut node = Node {
            top: Val::Percent(GAME_OVER_PANEL_TOP_DEFAULT_VALUE),
            ..Default::default()
        };

        let mut panel = GameOverPanel {
            timer: Timer::from_seconds(GAME_OVER_PANEL_ANIMATION_TIMER, TimerMode::Once),
            speed: GAME_OVER_PANEL_ANIMATION_SPEED_DEFAULT,
        };

        panel.timer.tick(Duration::from_secs_f32(
            GAME_OVER_PANEL_ANIMATION_TIMER / 2.0,
        ));

        let elapsed_before = panel.timer.elapsed_secs();

        GameOverPanel::show_logic(&mut node, &mut panel, &mut next_state, &time);

        let progress = elapsed_before / panel.timer.duration().as_secs_f32();
        let expected_top = GAME_OVER_PANEL_TOP_DEFAULT_VALUE - GAME_OVER_PANEL_TOP_END * progress;

        assert_eq!(node.top, Val::Percent(expected_top));
    }

    #[test]
    fn hide_logic_works() {
        let time = Time::default();

        let mut next_state = NextState::default();
        next_state.set(StateGameOverPanel::Hidding);

        let mut node = Node {
            top: Val::Percent(GAME_OVER_PANEL_TOP_END_REVERSED),
            ..Default::default()
        };

        let mut panel = GameOverPanel {
            timer: Timer::from_seconds(GAME_OVER_PANEL_ANIMATION_TIMER, TimerMode::Once),
            speed: GAME_OVER_PANEL_ANIMATION_SPEED_DEFAULT,
        };

        panel.timer.tick(Duration::from_secs_f32(
            GAME_OVER_PANEL_ANIMATION_TIMER / 2.0,
        ));

        let elapsed_before = panel.timer.elapsed_secs();

        GameOverPanel::hide_logic(&mut node, &mut panel, &mut next_state, &time);

        let progress = elapsed_before / panel.timer.duration().as_secs_f32();
        let expected_top = GAME_OVER_PANEL_TOP_END_REVERSED + GAME_OVER_PANEL_TOP_END * progress;

        assert_eq!(node.top, Val::Percent(expected_top));
    }
}
