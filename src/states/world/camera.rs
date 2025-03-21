use bevy::prelude::*;

#[derive(Default, States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CameraPositionState {
    #[default]
    Default,
    ToDefault,
    ToGameOver,
    GameOver,
}

impl CameraPositionState {
    pub fn when_default(state: Res<State<CameraPositionState>>) -> bool {
        CameraPositionState::Default == *state.get()
    }

    pub fn when_to_default(state: Res<State<CameraPositionState>>) -> bool {
        CameraPositionState::ToDefault == *state.get()
    }

    pub fn when_to_game_over(state: Res<State<CameraPositionState>>) -> bool {
        CameraPositionState::ToGameOver == *state.get()
    }

    pub fn when_game_over(state: Res<State<CameraPositionState>>) -> bool {
        CameraPositionState::GameOver == *state.get()
    }
}
