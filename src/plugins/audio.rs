use crate::{
    components::audio::AudioComponent,
    events::audio::ChangeVolumeEvent,
    resource::audio::{MusicChannel, SoundChannel},
    states::gameplay::GameState,
};
use bevy::prelude::*;
use bevy_kira_audio::{AudioApp, AudioPlugin};

/// The plugin for the audio system
pub struct RustydokuAudioPlugin;

impl Plugin for RustydokuAudioPlugin {
    fn build(&self, app: &mut App) {
        debug!("Building RustydokuMusicPlugin");

        trace!("Adding AudioPlugin");
        app.add_plugins(AudioPlugin);

        trace!("Adding event ChangeVolume");
        app.add_event::<ChangeVolumeEvent>();

        trace!("Adding audio sound channel");
        app.add_audio_channel::<SoundChannel>();

        trace!("Adding audio music channel");
        app.add_audio_channel::<MusicChannel>();

        trace!("Adding system startup");
        app.add_systems(Startup, (AudioComponent::set_up, AudioComponent::idle));

        trace!("Adding system update");
        app.add_systems(
            Update,
            (
                AudioComponent::mute,
                AudioComponent::change_volume_by_button,
                AudioComponent::denied_place,
                AudioComponent::read_change_volume,
                AudioComponent::read_combo,
                AudioComponent::mute_when_window_unfocused,
            ),
        );

        trace!("Adding system on exit and on enter");
        app.add_systems(OnExit(GameState::GameOver), AudioComponent::idle);

        trace!("Adding system on enter game over");
        app.add_systems(OnEnter(GameState::GameOver), AudioComponent::loss);

        trace!("Adding system on update game over when placed");
        app.add_systems(Update, AudioComponent::place.run_if(GameState::when_placed));
    }
}
