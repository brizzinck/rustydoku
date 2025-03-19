use crate::{
    components::music::MusicComponent,
    resource::music::{MusicChannel, MusicResource},
};
use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

impl MusicComponent {
    pub(crate) fn loss(music: Res<MusicResource>, audio: Res<AudioChannel<MusicChannel>>) {
        audio.stop();
        audio.play(music.get_lose_music()).looped();
    }
}
