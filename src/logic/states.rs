use crate::states::gameplay::GameState;
use bevy::prelude::*;

impl GameState {
    pub(crate) fn reset_state(mut state: ResMut<NextState<GameState>>) {
        state.set(GameState::Idle);
        trace!("Next state StateGame::Idle");
    }
}
