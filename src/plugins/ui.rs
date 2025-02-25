use crate::{
    components::ui::{
        button_restart::RestartButton,
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
        app.add_systems(Startup, (HeaderUI::spawn, GameOverPanel::spawn));
        app.add_systems(
            Update,
            (
                HeaderCurrentScoreText::update,
                RestartButton::handle,
                GameOverCurrentScoreText::update,
                GameOverMaxScoreText::update,
            )
                .chain(),
        );

        app.add_systems(
            OnEnter(StateGame::GameOver),
            (
                Score::update_max_score,
                HeaderUI::hide,
                GameOverPanel::set_show,
            )
                .chain(),
        );

        app.add_systems(OnExit(StateGame::GameOver), GameOverPanel::set_hide);

        app.add_systems(
            Update,
            GameOverPanel::show.run_if(StateGameOverPanel::when_showing),
        );

        app.add_systems(
            Update,
            GameOverPanel::hide.run_if(StateGameOverPanel::when_hidding),
        );
    }
}
