use bevy::prelude::*;
#[cfg(feature = "debug-inspector")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rustydoku::{
    components::{camera::Camera, figures::FigurePlugin, map::Map},
    logic::check_combination,
    resource::Score,
};

fn main() {
    let mut game = App::new();

    game.add_plugins(DefaultPlugins);

    game.insert_resource(ClearColor(Color::srgb(0.9, 0.9, 0.3)));
    game.insert_resource(Score::default());

    game.add_plugins(Map);
    game.add_plugins(Camera);
    game.add_plugins(FigurePlugin);
    game.add_systems(PostUpdate, check_combination);

    #[cfg(feature = "debug-inspector")]
    game.add_plugins(WorldInspectorPlugin::new());

    game.run();
}
