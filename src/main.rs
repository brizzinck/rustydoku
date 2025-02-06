use bevy::prelude::*;
#[cfg(feature = "debug-inspector")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rustydoku::components::camera::Camera;

fn main() {
    let mut game = App::new();

    game.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "RustyDoku".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    }));

    game.add_plugins(Camera);

    #[cfg(feature = "debug-inspector")]
    game.add_plugins(WorldInspectorPlugin::new());

    game.run();
}
