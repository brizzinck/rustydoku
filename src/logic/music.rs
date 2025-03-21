use crate::{
    components::music::MusicComponent,
    events::music::ChangeVolume,
    resource::music::{MusicChannel, MusicResource, SoundChannel},
};
use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

impl MusicComponent {
    pub fn set_up(
        music_channel: Res<AudioChannel<MusicChannel>>,
        sound_channel: Res<AudioChannel<SoundChannel>>,
        music: Res<MusicResource>,
    ) {
        trace!("Setting up music channel");
        music_channel.set_volume(music.get_volume_music());
        sound_channel.set_volume(music.get_volume_sound());
    }

    pub fn idle(music: Res<MusicResource>, audio: Res<AudioChannel<MusicChannel>>) {
        audio.stop();
        audio.play(music.get_idle_music()).looped();
        trace!("Playing idle music");
    }

    pub fn pause(
        input: Res<ButtonInput<KeyCode>>,
        mut music: ResMut<MusicResource>,
        event_write: EventWriter<ChangeVolume>,
    ) {
        if input.just_pressed(KeyCode::Space) {
            music.set_volume(0.0, event_write);
            trace!("Pausing music");
        }
    }

    pub fn read_change_volume(
        music_channel: Res<AudioChannel<MusicChannel>>,
        sound_channel: Res<AudioChannel<SoundChannel>>,
        mut event_reader: EventReader<ChangeVolume>,
    ) {
        if let Some(volume) = event_reader.read().last() {
            music_channel.set_volume(volume.music_volume);
            sound_channel.set_volume(volume.sound_volume);
            trace!(
                "Changing volume to music: {} sound: {}",
                volume.music_volume,
                volume.sound_volume
            );
        }
    }

    pub fn change_volume_by_button(
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut audio: ResMut<MusicResource>,
        event_write: EventWriter<ChangeVolume>,
    ) {
        if keyboard_input.just_pressed(KeyCode::Equal) {
            audio.up_volume(event_write);
            trace!("Increasing volume");
        } else if keyboard_input.just_pressed(KeyCode::Minus) {
            audio.down_volume(event_write);
            trace!("Decreasing volume");
        }
    }
}
