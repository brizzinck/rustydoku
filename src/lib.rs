use bevy::prelude::*;

#[cfg(feature = "debug-inspector")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use plugins::default::RustydokuDefault;
use plugins::game_zone::GameZonePlugin;
use plugins::logic::RustydokuLogicPlugin;
use plugins::ui::RustydokuUIPlugin;
use plugins::{camera::CameraPlugin, figure::FigurePlugin, map::MapPlugin};
use resource::figure_spawner::FigureSpawner;
use resource::map::Map;
use resource::score::Score;
use states::StateGame;

pub mod components;
pub mod logic;
pub mod plugins;
pub mod resource;
pub mod states;

pub fn run() {
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
    game.insert_resource(Map::default());

    game.insert_state(StateGame::default());

    game.run();
}
