use crate::{
    components::{music::MusicComponent, ui::button_restart::RestartButton},
    resource::music::{MusicResource, SoundChannel},
    states::{gameplay::StateGame, ui::restart_button::RestartButtonType},
};
use bevy::prelude::*;
use bevy_kira_audio::AudioChannel;

impl RestartButton {
    pub(crate) fn handle(
        mut interaction_query: Query<(&Interaction, &RestartButton), Changed<Interaction>>,
        mut state: ResMut<NextState<StateGame>>,
        music: Res<MusicResource>,
        sound_channel: Res<AudioChannel<SoundChannel>>,
    ) {
        for (interaction, button) in &mut interaction_query {
            if *interaction == Interaction::Pressed {
                MusicComponent::click(music, sound_channel);

                match button.restart_type {
                    RestartButtonType::Default => {
                        state.set(StateGame::DefaultRestart);
                        break;
                    }
                    RestartButtonType::GameOver => {
                        state.set(StateGame::GameOverRestart);
                        break;
                    }
                }
            }
        }
    }
}
