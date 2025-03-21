use crate::{
    components::music::MusicComponent,
    events::gameplay::Combo,
    resource::music::{MusicChannel, MusicResource, SoundChannel},
};
use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

impl MusicComponent {
    pub(crate) fn loss(music: Res<MusicResource>, audio: Res<AudioChannel<MusicChannel>>) {
        audio.stop();
        audio.play(music.get_lose_music()).looped();
    }

    pub(crate) fn read_combo(
        mut event_read: EventReader<Combo>,
        music: Res<MusicResource>,
        audio: Res<AudioChannel<SoundChannel>>,
    ) {
        if event_read.read().last().is_some() {
            audio.play(music.get_combo_sound());
            trace!("Playing combo sound");
        }
    }
}
