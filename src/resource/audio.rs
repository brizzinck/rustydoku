use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

use crate::{
    constants::{
        audio::assets::*,
        figure::{FIGURE_DENIED_PLACE_SOUND, FIGURE_PLACE_SOUND},
        ui::audio::*,
    },
    events::audio::ChangeVolumeEvent,
};

pub(crate) enum Volume {
    Mute(f64),
    Play(f64),
}

impl Volume {
    pub(crate) fn get_volume(&self) -> f64 {
        match self {
            Volume::Mute(volume) => *volume,
            Volume::Play(volume) => *volume,
        }
    }
}

#[derive(Resource)]
pub struct RustydokuAudioResource {
    volume: Volume,
    idle_music: Handle<AudioSource>,
    lose_music: Handle<AudioSource>,
    combo_sound: Handle<AudioSource>,
    place_sound: Handle<AudioSource>,
    denied_sound: Handle<AudioSource>,
    click_sound: Handle<AudioSource>,
}

impl RustydokuAudioResource {
    pub(crate) fn new(
        idle_music: Handle<AudioSource>,
        lose_music: Handle<AudioSource>,
        place_sound: Handle<AudioSource>,
        denied_sound: Handle<AudioSource>,
        click_sound: Handle<AudioSource>,
        combo_sound: Handle<AudioSource>,
    ) -> Self {
        Self {
            volume: Volume::Play(0.3),
            idle_music,
            lose_music,
            place_sound,
            denied_sound,
            click_sound,
            combo_sound,
        }
    }

    pub(crate) fn set_volume(&mut self, volume: f64, event_writer: EventWriter<ChangeVolumeEvent>) {
        self.volume = Volume::Play(volume);

        self.send_change_volume(event_writer);
    }

    pub(crate) fn up_volume(&mut self, event_writer: EventWriter<ChangeVolumeEvent>) {
        if self.volume.get_volume() == 1.0 {
            return;
        }

        let volume = self.volume.get_volume();
        self.volume = Volume::Play(1.0_f64.min(volume + 0.01));

        self.send_change_volume(event_writer);
    }

    pub(crate) fn down_volume(&mut self, event_writer: EventWriter<ChangeVolumeEvent>) {
        if self.volume.get_volume() == 0.0 {
            return;
        }

        let volume = self.volume.get_volume();
        self.volume = Volume::Play(0.0_f64.max(volume - 0.01));

        self.send_change_volume(event_writer);
    }

    fn send_change_volume(&mut self, mut event_writer: EventWriter<'_, ChangeVolumeEvent>) {
        event_writer.send(ChangeVolumeEvent {
            music_volume: self.get_volume_music(),
            sound_volume: self.get_volume_sound(),
        });
    }

    pub(crate) fn get_volume_sound(&self) -> f64 {
        match self.volume {
            Volume::Mute(_) => 0.0,
            Volume::Play(volume) => {
                if volume < 0.25 {
                    0.0
                } else {
                    volume
                }
            }
        }
    }

    pub(crate) fn get_volume_music(&self) -> f64 {
        match self.volume {
            Volume::Mute(_) => 0.0,
            Volume::Play(volume) => {
                if volume < 0.25 {
                    0.0
                } else {
                    volume - 0.25
                }
            }
        }
    }

    pub(crate) fn toggle_mute(&mut self, event_writer: EventWriter<ChangeVolumeEvent>) {
        match self.volume {
            Volume::Mute(volume) => self.volume = Volume::Play(volume),
            Volume::Play(volume) => self.volume = Volume::Mute(volume),
        }

        self.send_change_volume(event_writer);
    }

    pub(crate) fn get_idle_music(&self) -> Handle<AudioSource> {
        self.idle_music.clone()
    }

    pub(crate) fn get_lose_music(&self) -> Handle<AudioSource> {
        self.lose_music.clone()
    }

    pub(crate) fn get_combo_sound(&self) -> Handle<AudioSource> {
        self.combo_sound.clone()
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

    pub(crate) fn init(asset_server: &AssetServer) -> Self {
        let background_handle = asset_server.load(BACKGROUND_MUSIC);
        let player_handle = asset_server.load(LOSS_MUSIC);
        let place_sound = asset_server.load(FIGURE_PLACE_SOUND);
        let denied_place_sound = asset_server.load(FIGURE_DENIED_PLACE_SOUND);
        let click_sound = asset_server.load(BUTTON_CLICK_SOUND);
        let combo_sound = asset_server.load(COMBO_SOUND);

        RustydokuAudioResource::new(
            background_handle,
            player_handle,
            place_sound,
            denied_place_sound,
            click_sound,
            combo_sound,
        )
    }
}

#[derive(Resource, Component, Default, Clone)]
pub struct MusicChannel;

#[derive(Resource, Component, Default, Clone)]
pub struct SoundChannel;
