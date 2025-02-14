use crate::{
    logic::{check_combo::check_combination, check_game_over::check_game_over},
    states::StateGame,
};
use bevy::prelude::*;

pub struct RustydokuLogicPlugin;

impl Plugin for RustydokuLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(StateGame::CheckCombo), check_combination);
        app.add_systems(OnEnter(StateGame::CheckGameOver), check_game_over);
    }
}
