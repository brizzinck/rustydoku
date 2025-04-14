use crate::states::gameplay::GameState;
use bevy::prelude::*;

impl GameState {
    /// Resets the game state to [`GameState::Idle`], the default state of the game loop.
    ///
    /// It updates the [`NextState<GameState>`] resource accordingly and logs
    /// the transition for debugging purposes.
    ///
    /// # Parameters
    /// - `state`: A mutable reference to [`NextState<GameState>`], used to queue the state change.
    pub(crate) fn reset_state(mut state: ResMut<NextState<GameState>>) {
        state.set(GameState::Idle);
        trace!("Next state StateGame::Idle");
    }
}
