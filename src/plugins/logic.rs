use crate::{
    logic::gameplay::{check_combo::check_combination, check_game_over::check_game_over},
    states::gameplay::StateGame,
};
use bevy::prelude::*;

pub struct RustydokuLogicPlugin;

impl Plugin for RustydokuLogicPlugin {
    fn build(&self, app: &mut App) {
        debug!("Building RustydokuLogicPlugin");

        trace!("Adding systems to the app on state check combo");
        app.add_systems(OnEnter(StateGame::CheckCombo), check_combination);

        trace!("Adding systems to the app on state check game over");
        app.add_systems(OnEnter(StateGame::CheckGameOver), check_game_over);

        debug!("RustydokuLogicPlugin built");
    }
}
