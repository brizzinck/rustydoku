use crate::{
    components::music::MusicComponent,
    events::music::ChangeVolume,
    resource::music::{MusicChannel, MusicResource, SoundChannel},
    states::gameplay::StateGame,
};
use bevy::prelude::*;
use bevy_kira_audio::{AudioApp, AudioPlugin};

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin);

        app.add_event::<ChangeVolume>();

        app.add_audio_channel::<SoundChannel>();
        app.add_audio_channel::<MusicChannel>();

        app.add_systems(PreStartup, MusicResource::load_assets);

        app.add_systems(Startup, MusicComponent::set_up);
        app.add_systems(Startup, MusicComponent::idle);

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

        app.add_systems(OnExit(StateGame::GameOver), MusicComponent::idle);

        app.add_systems(OnEnter(StateGame::GameOver), MusicComponent::loss);

        app.add_systems(Update, MusicComponent::place.run_if(StateGame::when_placed));
    }
}
