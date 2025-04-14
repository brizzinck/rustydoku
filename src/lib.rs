use bevy::prelude::*;
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};

#[cfg(feature = "debug-inspector")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use plugins::audio::RustydokuAudioPlugin;
use plugins::default::RustydokuDefaultPlugin;
use plugins::firure_spawner::RustydokuFigureSpawnerPlugin;
use plugins::gameplay::RustydokuGameplayPlugin;
use plugins::logger::RustydokuLoggerPlugin;
use plugins::logic::RustydokuLogicPlugin;
use plugins::placeholer::RustydokuPlaceholderPlugin;
use plugins::resource::RustydokuResourcePlugin;
use plugins::ui::RustydokuUIPlugin;
use plugins::{
    camera::RustydokuCameraPlugin, figure::RustydokuFigurePlugin, map::RustydokuMapPlugin,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

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

    game.add_plugins(EmbeddedAssetPlugin {
        mode: PluginMode::ReplaceDefault,
    });
    game.add_plugins(RustydokuDefaultPlugin);
    game.add_plugins(RustydokuLoggerPlugin);
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

    #[cfg(feature = "debug-inspector")]
    game.add_plugins(WorldInspectorPlugin::new());

    game.run();
}

/// The main entry point for the game for WebAssembly builds
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn wasm_main() {
    run();
}

/// The main entry point for the game for Android native builds
#[cfg(target_os = "android")]
#[bevy_main]
fn main() {
    run();
}
