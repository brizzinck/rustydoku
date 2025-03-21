use crate::{
    components::{music::AudioComponent, ui::buttons::button_restart::ButtonRestart},
    resource::audio::{RustydokuAudioResource, SoundChannel},
    states::{gameplay::GameState, ui::button_restart::RestartButtonType},
};
use bevy::prelude::*;
use bevy_kira_audio::AudioChannel;

impl ButtonRestart {
    pub(crate) fn handle(
        mut interaction_query: Query<(&Interaction, &ButtonRestart), Changed<Interaction>>,
        mut state: ResMut<NextState<GameState>>,
        music: Res<RustydokuAudioResource>,
        sound_channel: Res<AudioChannel<SoundChannel>>,
    ) {
        for (interaction, button) in &mut interaction_query {
            if *interaction == Interaction::Pressed {
                AudioComponent::click(&music, sound_channel);

                match button.restart_type {
                    RestartButtonType::Default => {
                        state.set(GameState::DefaultRestart);
                        break;
                    }
                    RestartButtonType::GameOver => {
                        state.set(GameState::GameOverRestart);
                        break;
                    }
                }
            }
        }
    }
}
