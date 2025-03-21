use crate::{
    components::world::camera::RustyCamera2D,
    states::{
        gameplay::StateGame, ui::game_over_panel::StateGameOverPanel,
        world::camera::StateCameraPosition,
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
        app.add_systems(Startup, RustyCamera2D::spawn);

        trace!("Adding systems game over event for camera");
        app.add_systems(
            OnEnter(StateGame::GameOver),
            RustyCamera2D::set_camera_game_over,
        );

        trace!("Adding systems default event for camera");
        app.add_systems(
            OnEnter(StateGameOverPanel::Hidden),
            RustyCamera2D::set_camera_default,
        );

        trace!("Adding systems for camera position game over");
        app.add_systems(
            Update,
            RustyCamera2D::setting_game_over.run_if(StateCameraPosition::when_to_game_over),
        );

        trace!("Adding systems for camera position default");
        app.add_systems(
            Update,
            RustyCamera2D::setting_default.run_if(StateCameraPosition::when_to_default),
        );

        debug!("RustydokuCameraPlugin built");
    }
}
