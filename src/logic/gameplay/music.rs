use crate::{
    components::music::AudioComponent,
    events::gameplay::ComboEvent,
    resource::audio::{MusicChannel, RustydokuAudioResource, SoundChannel},
};
use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

impl AudioComponent {
    pub(crate) fn loss(music: Res<RustydokuAudioResource>, audio: Res<AudioChannel<MusicChannel>>) {
        audio.stop();
        audio.play(music.get_lose_music()).looped();
    }

    pub(crate) fn read_combo(
        mut event_read: EventReader<ComboEvent>,
        music: Res<RustydokuAudioResource>,
        audio: Res<AudioChannel<SoundChannel>>,
    ) {
        if event_read.read().last().is_some() {
            audio.play(music.get_combo_sound());
            trace!("Playing combo sound");
        }
    }
}
