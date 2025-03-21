use crate::{
    components::ui::buttons::button_audio::{AudioImage, ButtonAudio},
    events::audio::ChangeVolumeEvent,
    resource::audio::RustydokuAudioResource,
};
use bevy::prelude::*;

impl ButtonAudio {
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
