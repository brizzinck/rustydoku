use crate::{states::gameplay::StateGame, world::gameplay::Gameplay};
use bevy::prelude::*;

pub struct RustydokuLogicPlugin;

impl Plugin for RustydokuLogicPlugin {
    fn build(&self, app: &mut App) {
        debug!("Building RustydokuLogicPlugin");

        trace!("Adding systems to the app on state check combo");
        app.add_systems(OnEnter(StateGame::CheckCombo), Gameplay::check_combination);

        trace!("Adding systems to the app on state check game over");
        app.add_systems(OnEnter(StateGame::CheckGameOver), Gameplay::check_game_over);

        debug!("RustydokuLogicPlugin built");
    }
}
