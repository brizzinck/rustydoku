use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

use crate::{
    constants::{
        audio::{assets::*, AUDIO_DEFAULT_VOLUME},
        figure::{FIGURE_DENIED_PLACE_SOUND, FIGURE_PLACE_SOUND},
        ui::{
            assets::{AUDIO_OFF_BUTTON_IMAGE_PATH, AUDIO_ON_BUTTON_IMAGE_PATH},
            audio::assets::*,
        },
    },
    events::audio::ChangeVolumeEvent,
};

/// The music channel, for background music
#[derive(Resource, Component, Default, Clone)]
pub struct MusicChannel;

/// The sound channel, for sound effects
#[derive(Resource, Component, Default, Clone)]
pub struct SoundChannel;

/// The state of the volume
pub(crate) enum Volume {
    /// The volume is muted, contains the last volume before muting
    Mute(f64),
    /// The volume is playing, contains the current volume
    Play(f64),
}

impl Volume {
    /// Returns the current volume
    pub(crate) fn get_volume(&self) -> f64 {
        match self {
            Volume::Mute(volume) => *volume,
            Volume::Play(volume) => *volume,
        }
    }
}

/// The audio resource for the game
/// Contains the audio files for optimization and the current volume
#[derive(Resource)]
pub struct RustydokuAudioResource {
    /// The current volume
    volume: Volume,

    /// The idle music, which is played during the game (background music)
    idle_music: Handle<AudioSource>,

    /// The lose music, which is played when the player loses the game (background music)
    lose_music: Handle<AudioSource>,

    /// The combo sound, which is played when the player makes a combo
    combo_sound: Handle<AudioSource>,

    /// The place sound, which is played when the player places a figure
    place_sound: Handle<AudioSource>,

    /// The denied sound, which is played when the player tries to place a figure on an invalid position
    denied_sound: Handle<AudioSource>,

    /// The click sound, which is played when the player clicks a button
    click_sound: Handle<AudioSource>,

    /// The icon for the audio off button
    icon_off: Handle<Image>,

    /// The icon for the audio on button
    icon_on: Handle<Image>,
}

impl RustydokuAudioResource {
    /// Initializes the audio resource by loading all required audio files and icons using the `AssetServer`.
    ///
    /// # Parameters
    /// - `asset_server`: Reference to the [`AssetServer`] used to load assets.
    ///
    /// # Returns
    /// - A fully initialized [`RustydokuAudioResource`] containing all audio and icon handles.
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

    /// Increases the volume by `0.01`, unless it is already at the maximum (`1.0`).
    ///
    /// After updating the volume, this function emits a [`ChangeVolumeEvent`].
    ///
    /// # Parameters
    /// - `event_writer`: Event writer to notify listeners about the volume change.
    pub(crate) fn up_volume(&mut self, event_writer: EventWriter<ChangeVolumeEvent>) {
        if self.volume.get_volume() == 1.0 {
            return;
        }

        let volume = self.volume.get_volume();
        self.volume = Volume::Play(1.0_f64.min(volume + 0.01));

        self.send_change_volume(event_writer);
    }

    /// Decreases the volume by `0.01`, unless it is already at the minimum (`0.0`).
    ///
    /// After updating the volume, this function emits a [`ChangeVolumeEvent`].
    ///
    /// # Parameters
    /// - `event_writer`: Event writer to notify listeners about the volume change.
    pub(crate) fn down_volume(&mut self, event_writer: EventWriter<ChangeVolumeEvent>) {
        if self.volume.get_volume() == 0.0 {
            return;
        }

        let volume = self.volume.get_volume();
        self.volume = Volume::Play(0.0_f64.max(volume - 0.01));

        self.send_change_volume(event_writer);
    }

    /// Sends a [`ChangeVolumeEvent`] with the current sound and music volume levels.
    ///
    /// This method is called after changing the volume or toggling mute.
    ///
    /// # Parameters
    /// - `event_writer`: Event writer for sending volume change events.
    fn send_change_volume(&mut self, mut event_writer: EventWriter<'_, ChangeVolumeEvent>) {
        event_writer.send(ChangeVolumeEvent {
            music_volume: self.get_volume_music(),
            sound_volume: self.get_volume_sound(),
        });
    }

    /// Returns the current volume for sound effects.
    ///
    /// If the volume is below `0.25`, this returns `0.0` to effectively mute quiet effects.
    ///
    /// # Returns
    /// - A `f64` value representing the effective sound volume.
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

    /// Returns the current volume for background music.
    ///
    /// If the volume is below `0.25`, this returns `0.0`. Otherwise, it subtracts `0.25` to make background music quieter.
    ///
    /// # Returns
    /// - A `f64` value representing the effective music volume.
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

    /// Toggles the mute state.
    ///
    /// If the volume is currently muted, restores the previous volume.  
    /// If it's playing, stores the current volume and mutes it.  
    /// Then emits a [`ChangeVolumeEvent`] with the updated state.
    ///
    /// # Parameters
    /// - `event_writer`: Event writer used to notify about the volume change.
    pub(crate) fn toggle_mute(&mut self, event_writer: EventWriter<ChangeVolumeEvent>) {
        match self.volume {
            Volume::Mute(volume) => self.volume = Volume::Play(volume),
            Volume::Play(volume) => self.volume = Volume::Mute(volume),
        }

        self.send_change_volume(event_writer);
    }

    /// Returns the handle to the background (idle) music.
    ///
    /// # Returns
    /// - [`Handle<AudioSource>`] for idle music.
    pub(crate) fn get_idle_music(&self) -> Handle<AudioSource> {
        self.idle_music.clone()
    }

    /// Returns the handle to the lose music.
    ///
    /// # Returns
    /// - [`Handle<AudioSource>`] for game over music.
    pub(crate) fn get_lose_music(&self) -> Handle<AudioSource> {
        self.lose_music.clone()
    }

    /// Returns the handle to the combo sound effect (e.g. when multiple lines are cleared).
    ///
    /// # Returns
    /// - [`Handle<AudioSource>`] for the combo effect.
    pub(crate) fn get_combo_sound(&self) -> Handle<AudioSource> {
        self.combo_sound.clone()
    }

    /// Returns the handle to the placement sound effect (when a figure is placed).
    ///
    /// # Returns
    /// - [`Handle<AudioSource>`] for placement sound.
    pub(crate) fn get_place_sound(&self) -> Handle<AudioSource> {
        self.place_sound.clone()
    }

    /// Returns the handle to the denied placement sound effect.
    ///
    /// Played when the player attempts to place a figure in an invalid position.
    ///
    /// # Returns
    /// - [`Handle<AudioSource>`] for denied placement sound.
    pub(crate) fn get_denied_sound(&self) -> Handle<AudioSource> {
        self.denied_sound.clone()
    }

    /// Returns the handle to the click sound effect (e.g. button presses).
    ///
    /// # Returns
    /// - [`Handle<AudioSource>`] for UI clicks.
    pub(crate) fn get_click_sound(&self) -> Handle<AudioSource> {
        self.click_sound.clone()
    }

    /// Returns the icon handle for the "sound off" UI button.
    ///
    /// # Returns
    /// - [`Handle<Image>`] for the audio-off icon.
    pub(crate) fn get_icon_off(&self) -> Handle<Image> {
        self.icon_off.clone()
    }

    /// Returns the icon handle for the "sound on" UI button.
    ///
    /// # Returns
    /// - [`Handle<Image>`] for the audio-on icon.
    pub(crate) fn get_icon_on(&self) -> Handle<Image> {
        self.icon_on.clone()
    }
}
