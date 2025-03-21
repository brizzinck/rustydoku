use crate::{
    components::ui::{
        buttons::button_audio::ButtonAudio,
        buttons::button_restart::ButtonRestart,
        game_over_panel::{
            panel::GameOverPanelComponent,
            score_text::{GameOverCurrentScoreTextComponent, GameOverMaxScoreTextComponent},
        },
        header::{score_text::HeaderCurrentScoreTextComponent, HeaderUI},
    },
    resource::score::ScoreResource,
    states::{gameplay::GameState, ui::game_over_panel::GameOverPanelState},
};
use bevy::prelude::*;

/// The plugin for the UI
pub struct RustydokuUIPlugin;

impl Plugin for RustydokuUIPlugin {
    fn build(&self, app: &mut App) {
        debug!("Building RustydokuUIPlugin");

        trace!("Adding systems startup to RustydokuUIPlugin");
        app.add_systems(Startup, (HeaderUI::spawn, GameOverPanelComponent::spawn));

        trace!("Adding systems state on enter game over to RustydokuUIPlugin");
        app.add_systems(
            OnEnter(GameState::GameOver),
            (
                ScoreResource::update_max_score,
                HeaderUI::hide,
                GameOverPanelComponent::set_show,
            )
                .chain(),
        );

        trace!("Adding systems update to RustydokuUIPlugin");
        app.add_systems(
            Update,
            (
                HeaderCurrentScoreTextComponent::update,
                ButtonRestart::handle,
                ButtonAudio::handle,
                ButtonAudio::read_muted,
                GameOverCurrentScoreTextComponent::update,
                GameOverMaxScoreTextComponent::update,
            )
                .chain(),
        );

        trace!("Adding systems state on exit game over to RustydokuUIPlugin");
        app.add_systems(
            OnExit(GameState::GameOver),
            GameOverPanelComponent::set_hide,
        );

        trace!("Adding systems when showing game over to RustydokuUIPlugin");
        app.add_systems(
            Update,
            GameOverPanelComponent::show.run_if(GameOverPanelState::when_showing),
        );

        trace!("Adding systems when hidding game over to RustydokuUIPlugin");
        app.add_systems(
            Update,
            GameOverPanelComponent::hide.run_if(GameOverPanelState::when_hidding),
        );

        trace!("Adding state game over panel");
        app.insert_state(GameOverPanelState::default());

        debug!("RustydokuUIPlugin built");
    }
}
