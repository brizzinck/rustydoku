use bevy::prelude::*;

use bevy_embedded_assets::EmbeddedAssetPlugin;
#[cfg(feature = "debug-inspector")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use plugins::default::RustydokuDefaultPlugin;
use plugins::firure_spawner::RustydokuFigureSpawnerPlugin;
use plugins::gameplay::RustydokuGameplayPlugin;
use plugins::logic::RustydokuLogicPlugin;
use plugins::music::RustydokuAudioPlugin;
use plugins::placeholer::RustydokuPlaceholderPlugin;
use plugins::resource::RustydokuResourcePlugin;
use plugins::ui::RustydokuUIPlugin;
use plugins::{
    camera::RustydokuCameraPlugin, figure::RustydokuFigurePlugin, map::RustydokuMapPlugin,
};

pub mod components;
pub mod constants;
pub mod events;
pub mod logic;
pub mod plugins;
pub mod resource;
pub mod states;
pub mod world;

/// The main entry point for the game
pub fn run() {
    let mut game = App::new();

    game.add_plugins(RustydokuDefaultPlugin);
    game.add_plugins(RustydokuMapPlugin);
    game.add_plugins(RustydokuAudioPlugin);
    game.add_plugins(RustydokuCameraPlugin);
    game.add_plugins(RustydokuFigurePlugin);
    game.add_plugins(RustydokuFigureSpawnerPlugin);
    game.add_plugins(RustydokuPlaceholderPlugin);
    game.add_plugins(RustydokuUIPlugin);
    game.add_plugins(RustydokuLogicPlugin);
    game.add_plugins(RustydokuGameplayPlugin);
    game.add_plugins(RustydokuResourcePlugin);
    game.add_plugins(EmbeddedAssetPlugin::default());

    #[cfg(feature = "debug-inspector")]
    game.add_plugins(WorldInspectorPlugin::new());

    game.run();
}
