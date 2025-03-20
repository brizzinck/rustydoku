use crate::{
    components::music::MusicComponent,
    resource::music::{MusicResource, SoundChannel},
};
use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

impl MusicComponent {
    pub fn click(music: &MusicResource, sound_channel: Res<AudioChannel<SoundChannel>>) {
        sound_channel.play(music.get_click_sound());
    }
}
