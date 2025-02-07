use bevy::prelude::*;
use bevy::window::{Window, WindowResized};

#[cfg(feature = "debug-inspector")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use rustydoku::plugins::default::RustydokuDefault;
use rustydoku::plugins::{camera::CameraPlugin, figure::FigurePlugin, map::MapPlugin};
use rustydoku::{logic::check_combination, resource::Score};

fn main() {
    let mut game = App::new();

    game.add_plugins(RustydokuDefault);

    game.add_plugins(MapPlugin);
    game.add_plugins(CameraPlugin);
    game.add_plugins(FigurePlugin);

    #[cfg(feature = "debug-inspector")]
    game.add_plugins(WorldInspectorPlugin::new());

    game.insert_resource(ClearColor(Color::srgb(0.9, 0.9, 0.3)));
    game.insert_resource(Score::default());

    game.add_systems(PostUpdate, check_combination);
    game.add_systems(Update, adapt_to_window_size);

    game.run();
}

fn adapt_to_window_size(
    mut resize_events: EventReader<WindowResized>,
    mut windows: Query<&mut Window>,
) {
    for event in resize_events.read() {
        if let Ok(mut window) = windows.get_single_mut() {
            let width = event.width;
            let height = event.height;

            if width > height {
                window.resolution.set(height, width);
            } else {
                window.resolution.set(width, height);
            }
        }
    }
}
