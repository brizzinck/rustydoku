use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

use crate::{
    constants::{
        figure::{FIGURE_DENIED_PLACE_SOUND, FIGURE_PLACE_SOUND},
        music::assets::{BACKGROUND_MUSIC, LOSS_MUSIC},
        ui::music::BUTTON_CLICK_SOUND,
    },
    events::music::ChangeVolume,
};

#[derive(Resource)]
pub struct MusicResource {
    volume: f64,
    idle_music: Handle<AudioSource>,
    lose_music: Handle<AudioSource>,
    place_sound: Handle<AudioSource>,
    denied_sound: Handle<AudioSource>,
    click_sound: Handle<AudioSource>,
}

impl MusicResource {
    pub(crate) fn new(
        idle_music: Handle<AudioSource>,
        lose_music: Handle<AudioSource>,
        place_sound: Handle<AudioSource>,
        denied_sound: Handle<AudioSource>,
        click_sound: Handle<AudioSource>,
    ) -> Self {
        Self {
            volume: 0.3,
            idle_music,
            lose_music,
            place_sound,
            denied_sound,
            click_sound,
        }
    }

    pub(crate) fn set_volume(&mut self, volume: f64, event_writer: EventWriter<ChangeVolume>) {
        self.volume = volume;

        self.send_change_volume(event_writer);
    }

    pub(crate) fn up_volume(&mut self, event_writer: EventWriter<ChangeVolume>) {
        self.volume = 1.0_f64.min(self.volume + 0.01);

        self.send_change_volume(event_writer);
    }

    pub(crate) fn down_volume(&mut self, event_writer: EventWriter<ChangeVolume>) {
        self.volume = 0.0_f64.max(self.volume - 0.01);

        self.send_change_volume(event_writer);
    }

    fn send_change_volume(&mut self, mut event_writer: EventWriter<'_, ChangeVolume>) {
        event_writer.send(ChangeVolume {
            music_volume: self.get_volume_music(),
            sound_volume: self.get_volume_sound(),
        });
    }

    pub(crate) fn get_volume_sound(&self) -> f64 {
        if self.volume < 0.25 {
            0.0
        } else {
            self.volume
        }
    }

    pub(crate) fn get_volume_music(&self) -> f64 {
        if self.volume < 0.25 {
            0.0
        } else {
            self.volume - 0.25
        }
    }

    pub(crate) fn get_idle_music(&self) -> Handle<AudioSource> {
        self.idle_music.clone()
    }

    pub(crate) fn get_lose_music(&self) -> Handle<AudioSource> {
        self.lose_music.clone()
    }

    pub(crate) fn get_place_sound(&self) -> Handle<AudioSource> {
        self.place_sound.clone()
    }

    pub(crate) fn get_denied_sound(&self) -> Handle<AudioSource> {
        self.denied_sound.clone()
    }

    pub(crate) fn get_click_sound(&self) -> Handle<AudioSource> {
        self.click_sound.clone()
    }

    pub(crate) fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
        let background_handle = asset_server.load(BACKGROUND_MUSIC);
        let player_handle = asset_server.load(LOSS_MUSIC);
        let place_sound = asset_server.load(FIGURE_PLACE_SOUND);
        let denied_place_sound = asset_server.load(FIGURE_DENIED_PLACE_SOUND);
        let click_sound = asset_server.load(BUTTON_CLICK_SOUND);

        commands.insert_resource(MusicResource::new(
            background_handle,
            player_handle,
            place_sound,
            denied_place_sound,
            click_sound,
        ));
    }
}

#[derive(Resource, Component, Default, Clone)]
pub struct MusicChannel;

#[derive(Resource, Component, Default, Clone)]
pub struct SoundChannel;
