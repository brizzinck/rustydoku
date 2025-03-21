use bevy::prelude::*;

#[cfg(feature = "debug-inspector")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use events::figure::{
    FigureCanPlaced, FigureCantPlaced, FigureDeniedPlacing, FigureSpawned, FigureTriggerDragging,
    FigureTriggerUp,
};
use events::figure_spawner::SpawnFigure;
use events::gameplay::Combo;
use plugins::default::RustydokuDefaultPlugin;
use plugins::firure_spawner::RustydokuFigureSpawnerPlugin;
use plugins::gameplay::RustydokuGameplayPlugin;
use plugins::logic::RustydokuLogicPlugin;
use plugins::music::RustydokuMusicPlugin;
use plugins::placeholer::RustydokuPlaceholderPlugin;
use plugins::resource::RustydokuResourcePlugin;
use plugins::ui::RustydokuUIPlugin;
use plugins::{
    camera::RustydokuCameraPlugin, figure::RustydokuFigurePlugin, map::RustydokuMapPlugin,
};
use states::figure::placeholder::StatePlaceholderAnimation;
use states::gameplay::StateGame;
use states::ui::game_over_panel::StateGameOverPanel;
use states::world::camera::StateCameraPosition;

pub mod components;
pub mod constants;
pub mod events;
pub mod logic;
pub mod plugins;
pub mod resource;
pub mod states;
pub mod world;

pub fn run() {
    let mut game = App::new();

    game.add_event::<FigureTriggerDragging>();
    game.add_event::<FigureTriggerUp>();
    game.add_event::<FigureDeniedPlacing>();
    game.add_event::<FigureSpawned>();
    game.add_event::<FigureCantPlaced>();
    game.add_event::<FigureCanPlaced>();
    game.add_event::<SpawnFigure>();
    game.add_event::<Combo>();

    game.add_plugins(RustydokuDefaultPlugin);
    game.add_plugins(RustydokuMapPlugin);
    game.add_plugins(RustydokuMusicPlugin);
    game.add_plugins(RustydokuCameraPlugin);
    game.add_plugins(RustydokuFigurePlugin);
    game.add_plugins(RustydokuFigureSpawnerPlugin);
    game.add_plugins(RustydokuPlaceholderPlugin);
    game.add_plugins(RustydokuUIPlugin);
    game.add_plugins(RustydokuLogicPlugin);
    game.add_plugins(RustydokuGameplayPlugin);

    #[cfg(feature = "debug-inspector")]
    game.add_plugins(WorldInspectorPlugin::new());

    game.add_plugins(RustydokuResourcePlugin);

    game.insert_state(StateGame::default());
    game.insert_state(StateCameraPosition::default());
    game.insert_state(StateGameOverPanel::default());
    game.insert_state(StatePlaceholderAnimation::default());

    game.run();
}
