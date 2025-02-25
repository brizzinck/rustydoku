use crate::{
    logic::animation::camera::{setting_camera_default, setting_camera_game_over},
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
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Camera2d);
        });

        app.add_systems(OnEnter(StateGame::GameOver), set_camera_game_over);
        app.add_systems(OnEnter(StateGameOverPanel::Hidded), set_camera_default);

        app.add_systems(
            Update,
            setting_camera_game_over.run_if(StateCameraPosition::when_to_game_over),
        );

        app.add_systems(
            Update,
            setting_camera_default.run_if(StateCameraPosition::when_to_default),
        );
    }
}

fn set_camera_game_over(mut next_state: ResMut<NextState<StateCameraPosition>>) {
    next_state.set(StateCameraPosition::ToGameOver);
}

fn set_camera_default(mut next_state: ResMut<NextState<StateCameraPosition>>) {
    next_state.set(StateCameraPosition::ToDefault);
}
