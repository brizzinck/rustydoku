use crate::{events::gameplay::ComboEvent, states::gameplay::GameState, world::gameplay::Gameplay};
use bevy::prelude::*;

/// The plugin that contains the logic of loss and combination checking
pub struct RustydokuLogicPlugin;

impl Plugin for RustydokuLogicPlugin {
    fn build(&self, app: &mut App) {
        debug!("Building RustydokuLogicPlugin");

        trace!("Adding event combo");
        app.add_event::<ComboEvent>();

        trace!("Adding systems to the app on state check combo");
        app.add_systems(OnEnter(GameState::CheckCombo), Gameplay::check_combination);

        trace!("Adding systems to the app on state check game over");
        app.add_systems(OnEnter(GameState::CheckGameOver), Gameplay::check_game_over);

        trace!("Inserting state game");
        app.insert_state(GameState::default());

        debug!("RustydokuLogicPlugin built");
    }
}
