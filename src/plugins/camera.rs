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
    }
}
