use crate::constants::world::assets::BACKGROUND_IMAGE_PATH;
use crate::constants::world::background::BACKGROUND_DEFAULT_POSITION;
use crate::constants::world::window::{WINDOW_HEIGHT_SCALED_FACTOR, WINDOW_WIDTH_SCALED_FACTOR};
use bevy::prelude::*;
use bevy::window::WindowPlugin;

#[derive(Component)]
struct Background;

pub struct RustydokuDefault;

impl Plugin for RustydokuDefault {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(bevy::window::Window {
                title: "Rustydoku".to_string(),
                ..default()
            }),
            ..default()
        }));

        app.add_systems(Startup, spawn_background);
        app.add_systems(Update, fit_window_to_viewport);
    }
}

fn spawn_background(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(create_background(&assets));
}

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

fn create_background(assets: &Res<AssetServer>) -> impl Bundle {
    (
        Sprite {
            image: assets.load(BACKGROUND_IMAGE_PATH),
            ..default()
        },
        Transform {
            translation: BACKGROUND_DEFAULT_POSITION,
            scale: Vec3::ONE,
            ..default()
        },
        Background,
    )
}
