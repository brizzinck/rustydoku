use crate::constants::world::window::{WINDOW_HEIGHT_SCALED_FACTOR, WINDOW_WIDTH_SCALED_FACTOR};
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::WindowPlugin;

/// The default plugin window app for Rustydoku
pub struct RustydokuDefaultPlugin;

impl RustydokuDefaultPlugin {
    /// Fit the window to the viewport of all resolutions
    fn fit_window_to_viewport(
        mut windows: Query<&mut bevy::window::Window>,
        mut cameras: Query<&mut OrthographicProjection, With<Camera2d>>,
    ) {
        if let Ok(mut window) = windows.get_single_mut() {
            let width = window.resolution.width();
            let height = window.resolution.height();

            let (effective_width, effective_height) = if width > height {
                (height, height)
            } else {
                (width, height)
            };

            window.resolution.set(effective_width, effective_height);
            if let Ok(mut projection) = cameras.get_single_mut() {
                let scale_width = WINDOW_WIDTH_SCALED_FACTOR / effective_width;
                let scale_height = WINDOW_HEIGHT_SCALED_FACTOR / effective_height;
                projection.scale = scale_width.max(scale_height);
            }
        }
    }
}

impl Plugin for RustydokuDefaultPlugin {
    fn build(&self, app: &mut App) {
        debug!("Building RustydokuDefaultPlugin");

        trace!("Setting up default window");
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(bevy::window::Window {
                        title: "Rustydoku".to_string(),
                        ..default()
                    }),
                    ..default()
                })
                .build()
                .disable::<LogPlugin>(),
        );

        trace!("Setting up fit window to viewport system");
        app.add_systems(Update, Self::fit_window_to_viewport);

        debug!("RustydokuDefaultPlugin built");
    }
}
