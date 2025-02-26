use bevy::prelude::*;

#[cfg(feature = "debug-inspector")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use constants::world::background::BACKGROUND_CLEAR_COLOR;
use events::figure::{
    FigureCanPlaced, FigureCantPlaced, FigureDeniedPlacing, FigureSpawned, FigureTriggerDragging,
    FigureTriggerEnter, FigureTriggerUp,
};
use plugins::default::RustydokuDefault;
use plugins::gameplay::RustydokuGameplay;
use plugins::logic::RustydokuLogicPlugin;
use plugins::ui::RustydokuUIPlugin;
use plugins::{camera::CameraPlugin, figure::FigurePlugin, map::MapPlugin};
use resource::figure_spawner::FigureSpawner;
use resource::map::Map;
use resource::score::Score;
use resource::square::SquaresToDespawn;
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

    game.add_event::<FigureTriggerEnter>();
    game.add_event::<FigureTriggerDragging>();
    game.add_event::<FigureTriggerUp>();
    game.add_event::<FigureDeniedPlacing>();
    game.add_event::<FigureSpawned>();
    game.add_event::<FigureCantPlaced>();
    game.add_event::<FigureCanPlaced>();

    game.add_plugins(RustydokuDefault);
    game.add_plugins(MapPlugin);
    game.add_plugins(CameraPlugin);
    game.add_plugins(FigurePlugin);
    game.add_plugins(RustydokuUIPlugin);
    game.add_plugins(RustydokuLogicPlugin);
    game.add_plugins(RustydokuGameplay);

    #[cfg(feature = "debug-inspector")]
    game.add_plugins(WorldInspectorPlugin::new());

    game.insert_resource(ClearColor(BACKGROUND_CLEAR_COLOR));
    game.insert_resource(Score::default());
    game.insert_resource(FigureSpawner::default());
    game.insert_resource(Map::default());
    game.insert_resource(SquaresToDespawn::default());

    game.insert_state(StateGame::default());
    game.insert_state(StateCameraPosition::default());
    game.insert_state(StateGameOverPanel::default());
    game.insert_state(StatePlaceholderAnimation::default());

    game.run();
}
