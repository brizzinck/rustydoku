use bevy::prelude::*;

use crate::logic::resource::RustydokuResource;
use crate::states::gameplay::StateGame;

pub struct RustydokuResourcePlugin;

impl Plugin for RustydokuResourcePlugin {
    fn build(&self, app: &mut App) {
        debug!("Building RustydokuResourcePlugin");

        trace!("Adding systems to RustydokuResourcePlugin");
        app.add_systems(OnEnter(StateGame::InitResources), RustydokuResource::init);

        debug!("RustydokuResourcePlugin built");
    }
}
