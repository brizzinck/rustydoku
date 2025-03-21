use crate::{
    components::ui::buttons::button_audio::{AudioImage, ButtonAudio},
    events::audio::ChangeVolumeEvent,
    resource::audio::RustydokuAudioResource,
};
use bevy::prelude::*;

impl ButtonAudio {
    /// Handles interaction with the audio toggle button and mutes/unmutes the audio.
    ///
    /// When the button is pressed (i.e., `Interaction::Pressed`), this function toggles
    /// the mute state of the game’s audio by calling [`RustydokuAudioResource::toggle_mute`].
    /// A [`ChangeVolumeEvent`] is then emitted so other systems can react to the change.
    ///
    /// # Parameters
    /// - `interaction_query`: A query for button interactions that have changed.
    /// - `music`: A mutable reference to the audio resource.
    /// - `event_writer`: Event writer for sending [`ChangeVolumeEvent`]s.
    pub(crate) fn handle(
        interaction_query: Query<(&Interaction, &ButtonAudio), Changed<Interaction>>,
        mut music: ResMut<RustydokuAudioResource>,
        event_writer: EventWriter<ChangeVolumeEvent>,
    ) {
        if let Some((interaction, _)) = interaction_query.into_iter().last() {
            if *interaction == Interaction::Pressed {
                music.toggle_mute(event_writer);
            }
        }
    }

    /// Updates the image of the audio button based on the mute state.
    ///
    /// When a [`ChangeVolumeEvent`] is received, this function updates all UI elements
    /// marked with [`AudioImage`] to reflect the current audio state:
    /// - If volume is muted (both music and sound are 0.0), shows the "off" icon.
    /// - Otherwise, shows the "on" icon.
    ///
    /// # Parameters
    /// - `button_audio`: Query to access and update audio button images.
    /// - `event_reader`: Reader for [`ChangeVolumeEvent`]s.
    /// - `resource`: The audio resource providing access to icon handles.
    pub(crate) fn read_muted(
        mut button_audio: Query<&mut ImageNode, With<AudioImage>>,
        mut event_reader: EventReader<ChangeVolumeEvent>,
        resource: Res<RustydokuAudioResource>,
    ) {
        if let Some(event) = event_reader.read().last() {
            if event.music_volume == 0.0 && event.sound_volume == 0.0 {
                for mut image in button_audio.iter_mut() {
                    image.image = resource.get_icon_off();
                }
            } else {
                for mut image in button_audio.iter_mut() {
                    image.image = resource.get_icon_on();
                }
            }
        }
    }
}
