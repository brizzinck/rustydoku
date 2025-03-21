use crate::{
    components::music::MusicComponent,
    events::music::ChangeVolume,
    resource::music::{MusicChannel, SoundChannel},
    states::gameplay::StateGame,
};
use bevy::prelude::*;
use bevy_kira_audio::{AudioApp, AudioPlugin};

pub struct RustydokuMusicPlugin;

impl Plugin for RustydokuMusicPlugin {
    fn build(&self, app: &mut App) {
        debug!("Building RustydokuMusicPlugin");

        trace!("Adding AudioPlugin");
        app.add_plugins(AudioPlugin);

        trace!("Adding event ChangeVolume");
        app.add_event::<ChangeVolume>();

        trace!("Adding audio sound channel");
        app.add_audio_channel::<SoundChannel>();

        trace!("Adding audio music channel");
        app.add_audio_channel::<MusicChannel>();

        trace!("Adding system startup");
        app.add_systems(Startup, (MusicComponent::set_up, MusicComponent::idle));

        trace!("Adding system update");
        app.add_systems(
            Update,
            (
                MusicComponent::pause,
                MusicComponent::change_volume_by_button,
                MusicComponent::denied_place,
                MusicComponent::read_change_volume,
                MusicComponent::read_combo,
            ),
        );

        trace!("Adding system on exit and on enter");
        app.add_systems(OnExit(StateGame::GameOver), MusicComponent::idle);

        trace!("Adding system on enter game over");
        app.add_systems(OnEnter(StateGame::GameOver), MusicComponent::loss);

        trace!("Adding system on update game over when placed");
        app.add_systems(Update, MusicComponent::place.run_if(StateGame::when_placed));
    }
}
