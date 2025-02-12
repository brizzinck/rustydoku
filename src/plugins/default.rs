use bevy::prelude::*;
use bevy::window::{WindowPlugin, WindowResized, WindowResolution};

pub struct RustydokuDefault;

impl Plugin for RustydokuDefault {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(bevy::window::Window {
                resolution: WindowResolution::new(480., 800.),
                resizable: false,
                title: "Rustydoku".to_string(),
                ..default()
            }),
            ..default()
        }));

        app.add_systems(Update, enforce_portrait_mode);
    }
}

fn enforce_portrait_mode(
    mut resize_events: EventReader<WindowResized>,
    mut windows: Query<&mut bevy::window::Window>,
) {
    for event in resize_events.read() {
        if let Ok(mut window) = windows.get_single_mut() {
            let (width, height) = (event.width, event.height);

            if width > height {
                window.resolution.set(height, width);
            } else {
                window.resolution.set(width, height);
            }
        }
    }
}
