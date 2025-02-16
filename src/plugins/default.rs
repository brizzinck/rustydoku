use crate::constants::window::assets::BACKGROUND_IMAGE_PATH;
use bevy::prelude::*;
use bevy::window::{WindowPlugin, WindowResized};

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
        app.add_systems(Update, enforce_portrait_mode);
    }
}

fn spawn_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load(BACKGROUND_IMAGE_PATH),
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, -1.0),
            scale: Vec3::splat(1.0),
            ..Default::default()
        },
        Background,
    ));
}

#[derive(Component)]
struct Background;

fn enforce_portrait_mode(
    mut resize_events: EventReader<WindowResized>,
    mut windows: Query<&mut bevy::window::Window>,
    mut cameras: Query<&mut OrthographicProjection, With<Camera2d>>,
    mut backgrounds: Query<&mut Transform, With<Background>>,
) {
    for event in resize_events.read() {
        if let Ok(mut window) = windows.get_single_mut() {
            let (width, height) = (event.width, event.height);

            let (effective_width, effective_height) = if width > height {
                (height, height)
            } else {
                (width, height)
            };

            window.resolution.set(effective_width, effective_height);

            let scale_w = 420. / effective_width;
            let scale_h = 645. / effective_height;
            let required_scale = scale_w.max(scale_h);

            if let Ok(mut projection) = cameras.get_single_mut() {
                projection.scale = required_scale;
                info!("Updated camera scale: {}", projection.scale);
                if let Ok(mut background_transform) = backgrounds.get_single_mut() {
                    background_transform.scale = Vec3::new(1., 1., 1.0);
                }
            }
        }
    }
}
