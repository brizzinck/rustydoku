use crate::{
    components::music::AudioComponent,
    events::figure::FigureDeniedPlacingEvent,
    resource::audio::{RustydokuAudioResource, SoundChannel},
};
use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

impl AudioComponent {
    pub fn denied_place(
        music: Res<RustydokuAudioResource>,
        mut event: EventReader<FigureDeniedPlacingEvent>,
        sound: Res<AudioChannel<SoundChannel>>,
    ) {
        if event.read().last().is_some() {
            sound.play(music.get_denied_sound());
            trace!("Playing denied sound");
        }
    }

    pub fn place(music: Res<RustydokuAudioResource>, sound: Res<AudioChannel<SoundChannel>>) {
        sound.play(music.get_place_sound());
        trace!("Playing place sound");
    }
}
