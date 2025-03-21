use crate::{
    components::audio::AudioComponent,
    events::figure::FigureDeniedPlacingEvent,
    resource::audio::{RustydokuAudioResource, SoundChannel},
};
use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

impl AudioComponent {
    /// Plays the denied placement sound effect when a figure cannot be placed.
    ///
    /// This function listens for [`FigureDeniedPlacingEvent`]s and plays a sound effect
    /// to indicate invalid placement feedback to the player.
    ///
    /// # Parameters
    /// - `music`: The [`RustydokuAudioResource`] providing access to sound assets.
    /// - `event`: Reader for [`FigureDeniedPlacingEvent`] emitted by game logic.
    /// - `sound`: The sound channel used to play the denied sound effect.
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

    /// Plays the placement sound effect when a figure is successfully placed.
    ///
    /// This is triggered after a figure is dropped and accepted on the map.
    ///
    /// # Parameters
    /// - `music`: The audio resource providing the placement sound.
    /// - `sound`: The sound channel used to play sound effects.
    pub fn place(music: Res<RustydokuAudioResource>, sound: Res<AudioChannel<SoundChannel>>) {
        sound.play(music.get_place_sound());
        trace!("Playing place sound");
    }
}
