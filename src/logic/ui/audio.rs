use crate::{
    components::music::AudioComponent,
    resource::audio::{RustydokuAudioResource, SoundChannel},
};
use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

impl AudioComponent {
    pub fn click(music: &RustydokuAudioResource, sound_channel: Res<AudioChannel<SoundChannel>>) {
        sound_channel.play(music.get_click_sound());
    }
}
