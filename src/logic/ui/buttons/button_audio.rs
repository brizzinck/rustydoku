use crate::{
    components::ui::buttons::button_audio::{AudioButton, AudioImage},
    constants::ui::assets::{AUDIO_OFF_BUTTON_IMAGE_PATH, AUDIO_ON_BUTTON_IMAGE_PATH},
    events::music::ChangeVolume,
    resource::music::MusicResource,
};
use bevy::prelude::*;

impl AudioButton {
    pub(crate) fn handle(
        interaction_query: Query<(&Interaction, &AudioButton), Changed<Interaction>>,
        mut music: ResMut<MusicResource>,
        event_writer: EventWriter<ChangeVolume>,
    ) {
        if let Some((interaction, _)) = interaction_query.into_iter().last() {
            if *interaction == Interaction::Pressed {
                music.toggle_mute(event_writer);
            }
        }
    }

    pub(crate) fn read_muted(
        mut button_audio: Query<&mut ImageNode, With<AudioImage>>,
        mut event_reader: EventReader<ChangeVolume>,
        assets: Res<AssetServer>,
    ) {
        if let Some(event) = event_reader.read().last() {
            if event.music_volume == 0.0 && event.sound_volume == 0.0 {
                for mut image in button_audio.iter_mut() {
                    image.image = assets.load(AUDIO_OFF_BUTTON_IMAGE_PATH);
                }
            } else {
                for mut image in button_audio.iter_mut() {
                    image.image = assets.load(AUDIO_ON_BUTTON_IMAGE_PATH);
                }
            }
        }
    }
}
