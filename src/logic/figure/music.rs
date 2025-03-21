use crate::{
    components::music::MusicComponent,
    events::figure::FigureDeniedPlacing,
    resource::music::{MusicResource, SoundChannel},
};
use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

impl MusicComponent {
    pub fn denied_place(
        music: Res<MusicResource>,
        mut event: EventReader<FigureDeniedPlacing>,
        sound: Res<AudioChannel<SoundChannel>>,
    ) {
        if event.read().last().is_some() {
            sound.play(music.get_denied_sound());
            trace!("Playing denied sound");
        }
    }

    pub fn place(music: Res<MusicResource>, sound: Res<AudioChannel<SoundChannel>>) {
        sound.play(music.get_place_sound());
        trace!("Playing place sound");
    }
}
