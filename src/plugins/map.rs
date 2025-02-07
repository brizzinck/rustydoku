use crate::components::map::generate_map;
use bevy::app::{Plugin, Startup};
#[cfg(feature = "debug-inspector")]
use bevy_inspector_egui::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, generate_map);
        #[cfg(feature = "debug-inspector")]
        {
            use crate::components::map::Tile;
            app.register_type::<Tile>();
        }
    }
}
