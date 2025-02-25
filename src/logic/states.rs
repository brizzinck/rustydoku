use crate::states::gameplay::StateGame;
use bevy::prelude::*;

impl StateGame {
    pub(crate) fn reset_state(mut state: ResMut<NextState<StateGame>>) {
        state.set(StateGame::Idle);
    }
}
