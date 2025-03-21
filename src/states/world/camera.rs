use bevy::prelude::*;

/// The state of the camera animation position
#[derive(Default, States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CameraPositionState {
    /// Default camera position in the game
    #[default]
    Default,
    /// Transition to the default camera position
    ToDefault,
    /// Transition to the game over camera position
    ToGameOver,
    /// Game over camera position
    GameOver,
}

impl CameraPositionState {
    /// Returns true if the state is `Default`
    pub fn when_default(state: Res<State<CameraPositionState>>) -> bool {
        CameraPositionState::Default == *state.get()
    }

    /// Returns true if the state is `ToDefault`
    pub fn when_to_default(state: Res<State<CameraPositionState>>) -> bool {
        CameraPositionState::ToDefault == *state.get()
    }

    /// Returns true if the state is `ToGameOver`
    pub fn when_to_game_over(state: Res<State<CameraPositionState>>) -> bool {
        CameraPositionState::ToGameOver == *state.get()
    }

    /// Returns true if the state is `GameOver`
    pub fn when_game_over(state: Res<State<CameraPositionState>>) -> bool {
        CameraPositionState::GameOver == *state.get()
    }
}
