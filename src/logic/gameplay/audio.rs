use crate::{
    components::audio::AudioComponent,
    events::gameplay::ComboEvent,
    resource::audio::{MusicChannel, RustydokuAudioResource, SoundChannel},
};
use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

impl AudioComponent {
    /// Plays the loss music when the game is lost.
    ///
    /// This function stops any currently playing background music and starts
    /// the loss music in a loop using the music channel.
    ///
    /// Typically triggered when transitioning into the [`GameState::GameOver`] state.
    ///
    /// # Parameters
    /// - `music`: The [`RustydokuAudioResource`] containing audio handles.
    /// - `audio`: The [`AudioChannel<MusicChannel>`] used to control background music playback.
    pub(crate) fn loss(music: Res<RustydokuAudioResource>, audio: Res<AudioChannel<MusicChannel>>) {
        audio.stop();
        audio.play(music.get_lose_music()).looped();
    }

    /// Plays the combo sound when a [`ComboEvent`] is triggered.
    ///
    /// This provides auditory feedback when the player clears a full row, column, or 3x3 block.
    ///
    /// # Parameters
    /// - `event_read`: Reader for [`ComboEvent`]s emitted by the game logic.
    /// - `music`: The audio resource containing sound effect handles.
    /// - `audio`: The [`AudioChannel<SoundChannel>`] used for playing sound effects.
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
