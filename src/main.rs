use bevy::prelude::*;

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

    game.run();
}
