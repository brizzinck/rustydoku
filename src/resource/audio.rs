use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

use crate::{
    constants::{
        audio::{assets::*, AUDIO_DEFAULT_VOLUME},
        figure::{FIGURE_DENIED_PLACE_SOUND, FIGURE_PLACE_SOUND},
        ui::{
            assets::{AUDIO_OFF_BUTTON_IMAGE_PATH, AUDIO_ON_BUTTON_IMAGE_PATH},
            audio::*,
        },
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
    icon_off: Handle<Image>,
    icon_on: Handle<Image>,
}

impl RustydokuAudioResource {
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

    pub(crate) fn get_icon_off(&self) -> Handle<Image> {
        self.icon_off.clone()
    }

    pub(crate) fn get_icon_on(&self) -> Handle<Image> {
        self.icon_on.clone()
    }

    pub(crate) fn init(asset_server: &AssetServer) -> Self {
        RustydokuAudioResource {
            volume: Volume::Play(AUDIO_DEFAULT_VOLUME),
            idle_music: asset_server.load(BACKGROUND_MUSIC),
            lose_music: asset_server.load(LOSS_MUSIC),
            place_sound: asset_server.load(FIGURE_PLACE_SOUND),
            denied_sound: asset_server.load(FIGURE_DENIED_PLACE_SOUND),
            click_sound: asset_server.load(BUTTON_CLICK_SOUND),
            combo_sound: asset_server.load(COMBO_SOUND),
            icon_off: asset_server.load(AUDIO_OFF_BUTTON_IMAGE_PATH),
            icon_on: asset_server.load(AUDIO_ON_BUTTON_IMAGE_PATH),
        }
    }
}

#[derive(Resource, Component, Default, Clone)]
pub struct MusicChannel;

#[derive(Resource, Component, Default, Clone)]
pub struct SoundChannel;
