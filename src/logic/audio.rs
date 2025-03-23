use crate::{
    components::audio::AudioComponent,
    events::audio::ChangeVolumeEvent,
    resource::audio::{MusicChannel, RustydokuAudioResource, SoundChannel},
};
use bevy::{prelude::*, window::WindowFocused};
use bevy_kira_audio::{AudioChannel, AudioControl};

impl AudioComponent {
    /// Initializes the music and sound channels with volume values from the audio resource.
    ///
    /// This is typically called once during setup to ensure audio channels respect
    /// the current volume configuration from [`RustydokuAudioResource`].
    ///
    /// # Parameters
    /// - `music_channel`: The channel used for background music.
    /// - `sound_channel`: The channel used for sound effects.
    /// - `audio`: The resource storing volume levels and audio handles.
    pub fn set_up(
        music_channel: Res<AudioChannel<MusicChannel>>,
        sound_channel: Res<AudioChannel<SoundChannel>>,
        audio: Res<RustydokuAudioResource>,
    ) {
        trace!("Setting up music channel");
        music_channel.set_volume(audio.get_volume_music());
        sound_channel.set_volume(audio.get_volume_sound());
    }

    /// Plays the idle (background) music in a loop.
    ///
    /// This is typically used when the main game starts or restarts.
    ///
    /// # Parameters
    /// - `audio_resource`: Resource that provides the idle music handle.
    /// - `music_channel`: Audio channel used to control music playback.
    pub fn idle(
        rustydoku_audio: Res<RustydokuAudioResource>,
        audio: Res<AudioChannel<MusicChannel>>,
    ) {
        audio.stop();
        audio.play(rustydoku_audio.get_idle_music()).looped();
        trace!("Playing idle music");
    }

    /// Toggles mute on or off when the spacebar is pressed.
    ///
    /// This function updates the volume state in the audio resource and emits a
    /// [`ChangeVolumeEvent`] so systems can react accordingly.
    ///
    /// # Parameters
    /// - `input`: Input resource to detect key presses.
    /// - `audio`: Mutable reference to the audio resource.
    /// - `event_write`: Event writer to notify other systems of the volume change.
    pub fn mute(
        input: Res<ButtonInput<KeyCode>>,
        mut audio: ResMut<RustydokuAudioResource>,
        event_write: EventWriter<ChangeVolumeEvent>,
    ) {
        if input.just_pressed(KeyCode::Space) {
            audio.toggle_mute(event_write);
            trace!("Pausing music");
        }
    }

    /// Mutes the audio when the window is unfocused and unmutes when focused if not full muted.
    ///
    /// This function listens for [`WindowFocused`] events and mutes the audio when the window
    /// is unfocused. When the window is focused again, the audio is unmuted.
    ///
    /// # Parameters
    /// - `window_focus_events`: Event reader for window focus events.
    /// - `audio`: Mutable reference to the audio resource.
    /// - `event_write`: Event writer to notify other systems of the volume change.
    pub fn mute_when_window_unfocused(
        mut window_focus_events: EventReader<WindowFocused>,
        mut audio: ResMut<RustydokuAudioResource>,
        event_write: EventWriter<ChangeVolumeEvent>,
    ) {
        if let Some(event) = window_focus_events.read().next() {
            if event.focused {
                audio.window_un_mute(event_write);
            } else {
                audio.window_mute(event_write);
            }
        }
    }

    /// Listens for [`ChangeVolumeEvent`]s and updates channel volumes accordingly.
    ///
    /// Only the last event (if multiple are present in the same frame) is processed.
    ///
    /// # Parameters
    /// - `music_channel`: Audio channel for background music.
    /// - `sound_channel`: Audio channel for sound effects.
    /// - `event_reader`: Event reader for incoming volume change events.
    pub fn read_change_volume(
        music_channel: Res<AudioChannel<MusicChannel>>,
        sound_channel: Res<AudioChannel<SoundChannel>>,
        mut event_reader: EventReader<ChangeVolumeEvent>,
    ) {
        if let Some(volume) = event_reader.read().last() {
            music_channel.set_volume(volume.music_volume);
            sound_channel.set_volume(volume.sound_volume);
            trace!(
                "Changing volume to music: {} sound: {}",
                volume.music_volume,
                volume.sound_volume
            );
        }
    }

    /// Changes volume based on keyboard input: `=` to increase and `-` to decrease.
    ///
    /// After adjusting the volume, a [`ChangeVolumeEvent`] is emitted to keep systems updated.
    ///
    /// # Parameters
    /// - `keyboard_input`: Resource for keyboard input tracking.
    /// - `audio`: Mutable reference to the audio resource.
    /// - `event_write`: Event writer to notify volume change.
    pub fn change_volume_by_button(
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut audio: ResMut<RustydokuAudioResource>,
        event_write: EventWriter<ChangeVolumeEvent>,
    ) {
        if keyboard_input.just_pressed(KeyCode::Equal) {
            audio.up_volume(event_write);
            trace!("Increasing volume");
        } else if keyboard_input.just_pressed(KeyCode::Minus) {
            audio.down_volume(event_write);
            trace!("Decreasing volume");
        }
    }
}
