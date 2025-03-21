use bevy::prelude::*;

use crate::logic::resource::RustydokuResource;
use crate::states::gameplay::GameState;

pub struct RustydokuResourcePlugin;

impl Plugin for RustydokuResourcePlugin {
    fn build(&self, app: &mut App) {
        debug!("Building RustydokuResourcePlugin");

        trace!("Adding systems to RustydokuResourcePlugin");
        app.add_systems(OnEnter(GameState::InitResources), RustydokuResource::init);

        debug!("RustydokuResourcePlugin built");
    }
}
