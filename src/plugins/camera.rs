use crate::{
    components::world::camera::RustyCamera2DComponent,
    states::{
        gameplay::GameState, ui::game_over_panel::GameOverPanelState,
        world::camera::CameraPositionState,
    },
};
use bevy::{
    app::{Plugin, Startup},
    prelude::*,
};

pub struct RustydokuCameraPlugin;

impl Plugin for RustydokuCameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        debug!("Building RustydokuCameraPlugin");

        trace!("Spawning RustyCamera2D");
        app.add_systems(Startup, RustyCamera2DComponent::spawn);

        trace!("Adding systems game over event for camera");
        app.add_systems(
            OnEnter(GameState::GameOver),
            RustyCamera2DComponent::set_camera_game_over,
        );

        trace!("Adding systems default event for camera");
        app.add_systems(
            OnEnter(GameOverPanelState::Hidden),
            RustyCamera2DComponent::set_camera_default,
        );

        trace!("Adding systems for camera position game over");
        app.add_systems(
            Update,
            RustyCamera2DComponent::setting_game_over
                .run_if(CameraPositionState::when_to_game_over),
        );

        trace!("Adding systems for camera position default");
        app.add_systems(
            Update,
            RustyCamera2DComponent::setting_default.run_if(CameraPositionState::when_to_default),
        );

        trace!("Inserting state camera position");
        app.insert_state(CameraPositionState::default());

        debug!("RustydokuCameraPlugin built");
    }
}
