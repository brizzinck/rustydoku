use crate::{
    components::camera::RustyCamera2D,
    states::{
        gameplay::StateGame, ui::game_over_panel::StateGameOverPanel,
        world::camera::StateCameraPosition,
    },
};
use bevy::{
    app::{Plugin, Startup},
    prelude::*,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, RustyCamera2D::spawn);

        app.add_systems(
            OnEnter(StateGame::GameOver),
            RustyCamera2D::set_camera_game_over,
        );

        app.add_systems(
            OnEnter(StateGameOverPanel::Hidden),
            RustyCamera2D::set_camera_default,
        );

        app.add_systems(
            Update,
            RustyCamera2D::setting_game_over.run_if(StateCameraPosition::when_to_game_over),
        );

        app.add_systems(
            Update,
            RustyCamera2D::setting_default.run_if(StateCameraPosition::when_to_default),
        );
    }
}
