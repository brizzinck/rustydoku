#[cfg(not(target_arch = "wasm32"))]
use crate::constants::world::window::WINDOW_ICON_PATH;
use crate::constants::world::window::{WINDOW_HEIGHT_SCALED_FACTOR, WINDOW_WIDTH_SCALED_FACTOR};
use bevy::log::LogPlugin;
use bevy::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use bevy::window::PrimaryWindow;
use bevy::window::WindowPlugin;
#[cfg(not(target_arch = "wasm32"))]
use bevy::winit::WinitWindows;
#[cfg(not(target_arch = "wasm32"))]
use winit::window::Icon;

/// The default plugin window app for Rustydoku
pub struct RustydokuDefaultPlugin;

impl RustydokuDefaultPlugin {
    /// Fit the window to the viewport of all resolutions
    ///
    /// Parameters:
    /// - `windows`: Query<Window> - The window query
    /// - `cameras`: Query<OrthographicProjection> - The camera query
    fn fit_window_to_viewport(
        mut windows: Query<&mut bevy::window::Window>,
        mut cameras: Query<&mut OrthographicProjection, With<Camera2d>>,
    ) {
        if let Ok(window) = windows.get_single_mut() {
            let width = window.resolution.width();
            let height = window.resolution.height();

            if let Ok(mut projection) = cameras.get_single_mut() {
                let scale_width = WINDOW_WIDTH_SCALED_FACTOR / width;
                let scale_height = WINDOW_HEIGHT_SCALED_FACTOR / height;
                projection.scale = scale_width.max(scale_height);
            }
        }
    }

    // Set the window icon
    //
    // Parameters:
    // - `windows`: NonSend<WinitWindows> - The windows query
    // - `primary`: Query<Entity, With<PrimaryWindow>> - The primary window query
    #[cfg(not(target_arch = "wasm32"))]
    fn set_window_icon(
        windows: NonSend<WinitWindows>,
        primary: Query<Entity, With<PrimaryWindow>>,
    ) {
        if let Ok(primary_entity) = primary.get_single() {
            if let Some(primary_window) = windows.get_window(primary_entity) {
                let (icon_rgba, icon_width, icon_height) = {
                    let image = image::load_from_memory(WINDOW_ICON_PATH)
                        .expect("Failed to load embedded icon")
                        .into_rgba8();

                    let (width, height) = image.dimensions();
                    let rgba = image.into_raw();

                    (rgba, width, height)
                };

                let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height)
                    .expect("Failed to create icon");

                primary_window.set_window_icon(Some(icon));
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
                        canvas: Some("#rustycanvas".into()),
                        ..default()
                    }),
                    ..default()
                })
                .build()
                .disable::<LogPlugin>(),
        );

        #[cfg(not(target_arch = "wasm32"))]
        {
            trace!("Setting up set window icon system");
            app.add_systems(Startup, Self::set_window_icon);
        }

        trace!("Setting up fit window to viewport system");
        app.add_systems(Update, Self::fit_window_to_viewport);

        debug!("RustydokuDefaultPlugin built");
    }
}
