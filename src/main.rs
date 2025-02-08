use bevy::prelude::*;

#[cfg(feature = "debug-inspector")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use rustydoku::plugins::default::RustydokuDefault;
use rustydoku::plugins::game_zone::GameZonePlugin;
use rustydoku::plugins::logic::RustydokuLogicPlugin;
use rustydoku::plugins::ui::RustydokuUIPlugin;
use rustydoku::plugins::{camera::CameraPlugin, figure::FigurePlugin, map::MapPlugin};
use rustydoku::resource::figure_spawner::FigureSpawner;
use rustydoku::resource::score::Score;

fn main() {
    let mut game = App::new();

    game.add_plugins(RustydokuDefault);
    game.add_plugins(MapPlugin);
    game.add_plugins(CameraPlugin);
    game.add_plugins(FigurePlugin);
    game.add_plugins(GameZonePlugin);
    game.add_plugins(RustydokuUIPlugin);
    game.add_plugins(RustydokuLogicPlugin);
    #[cfg(feature = "debug-inspector")]
    game.add_plugins(WorldInspectorPlugin::new());

    game.insert_resource(ClearColor(Color::srgb(0.9, 0.9, 0.3)));
    game.insert_resource(Score::default());
    game.insert_resource(FigureSpawner::default());

    game.run();
}
