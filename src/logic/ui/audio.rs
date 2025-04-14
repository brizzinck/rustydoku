use crate::{
    components::audio::AudioComponent,
    resource::audio::{RustydokuAudioResource, SoundChannel},
};
use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

impl AudioComponent {
    /// Plays the click sound effect using the sound channel.
    ///
    /// This method is typically called in response to UI interactions (e.g. button clicks).
    ///
    /// # Parameters
    /// - `music`: Reference to the [`RustydokuAudioResource`] containing audio handles.
    /// - `sound_channel`: The Bevy Kira audio channel used for playing sound effects.
    pub fn click(music: &RustydokuAudioResource, sound_channel: Res<AudioChannel<SoundChannel>>) {
        sound_channel.play(music.get_click_sound());
    }
}
