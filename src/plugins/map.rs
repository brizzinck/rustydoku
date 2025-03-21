use crate::{resource::map::MapComponent, states::gameplay::GameState};
use bevy::prelude::*;
#[cfg(feature = "debug-inspector")]
use bevy_inspector_egui::prelude::*;

/// A plugin that adds the map to the game
pub struct RustydokuMapPlugin;

impl Plugin for RustydokuMapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        debug!("Building RustydokuMapPlugin");

        trace!("Adding systems to AppBuilder");
        app.add_systems(
            OnEnter(GameState::GenerateWorld),
            MapComponent::generate_map,
        );
        #[cfg(feature = "debug-inspector")]
        {
            use crate::components::world::map::TileComponent;
            app.register_type::<Tile>();
        }

        debug!("RustydokuMapPlugin built");
    }
}
