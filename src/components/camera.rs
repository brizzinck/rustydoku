use bevy::{
    app::{Plugin, Startup},
    prelude::*,
};

pub struct Camera;

impl Plugin for Camera {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Camera2dBundle::default());
        });
    }
}
