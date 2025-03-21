use crate::{resource::map::Map, states::gameplay::StateGame};
use bevy::prelude::*;
#[cfg(feature = "debug-inspector")]
use bevy_inspector_egui::prelude::*;

pub struct RustydokuMapPlugin;

impl Plugin for RustydokuMapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        debug!("Building RustydokuMapPlugin");

        trace!("Adding systems to AppBuilder");
        app.add_systems(OnEnter(StateGame::GenerateWorld), Map::generate_map);
        #[cfg(feature = "debug-inspector")]
        {
            use crate::components::world::map::Tile;
            app.register_type::<Tile>();
        }

        debug!("RustydokuMapPlugin built");
    }
}
