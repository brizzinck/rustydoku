use crate::{components::map::generate_map, states::gameplay::StateGame};
use bevy::{app::Plugin, prelude::OnEnter};
#[cfg(feature = "debug-inspector")]
use bevy_inspector_egui::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(StateGame::GenerateWorld), generate_map);
        #[cfg(feature = "debug-inspector")]
        {
            use crate::components::map::Tile;
            app.register_type::<Tile>();
        }
    }
}
