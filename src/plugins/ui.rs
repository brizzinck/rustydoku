use crate::{
    components::ui::{
        buttons::button_audio::AudioButton,
        buttons::button_restart::RestartButton,
        game_over_panel::{
            panel::GameOverPanel,
            score_text::{GameOverCurrentScoreText, GameOverMaxScoreText},
        },
        header::{score_text::HeaderCurrentScoreText, HeaderUI},
    },
    resource::score::Score,
    states::{gameplay::StateGame, ui::game_over_panel::StateGameOverPanel},
};
use bevy::prelude::*;

pub struct RustydokuUIPlugin;

impl Plugin for RustydokuUIPlugin {
    fn build(&self, app: &mut App) {
        debug!("Building RustydokuUIPlugin");

        trace!("Adding systems startup to RustydokuUIPlugin");
        app.add_systems(Startup, (HeaderUI::spawn, GameOverPanel::spawn));

        trace!("Adding systems state on enter game over to RustydokuUIPlugin");
        app.add_systems(
            OnEnter(StateGame::GameOver),
            (
                Score::update_max_score,
                HeaderUI::hide,
                GameOverPanel::set_show,
            )
                .chain(),
        );

        trace!("Adding systems update to RustydokuUIPlugin");
        app.add_systems(
            Update,
            (
                HeaderCurrentScoreText::update,
                RestartButton::handle,
                AudioButton::handle,
                AudioButton::read_muted,
                GameOverCurrentScoreText::update,
                GameOverMaxScoreText::update,
            )
                .chain(),
        );

        trace!("Adding systems state on exit game over to RustydokuUIPlugin");
        app.add_systems(OnExit(StateGame::GameOver), GameOverPanel::set_hide);

        trace!("Adding systems when showing game over to RustydokuUIPlugin");
        app.add_systems(
            Update,
            GameOverPanel::show.run_if(StateGameOverPanel::when_showing),
        );

        trace!("Adding systems when hidding game over to RustydokuUIPlugin");
        app.add_systems(
            Update,
            GameOverPanel::hide.run_if(StateGameOverPanel::when_hidding),
        );

        debug!("RustydokuUIPlugin built");
    }
}
