use bevy::prelude::*;

#[derive(Default, States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum StateCameraPosition {
    #[default]
    Default,
    ToDefault,
    ToGameOver,
    GameOver,
}

impl StateCameraPosition {
    pub fn when_default(state: Res<State<StateCameraPosition>>) -> bool {
        StateCameraPosition::Default == *state.get()
    }

    pub fn when_to_default(state: Res<State<StateCameraPosition>>) -> bool {
        StateCameraPosition::ToDefault == *state.get()
    }

    pub fn when_to_game_over(state: Res<State<StateCameraPosition>>) -> bool {
        StateCameraPosition::ToGameOver == *state.get()
    }

    pub fn when_game_over(state: Res<State<StateCameraPosition>>) -> bool {
        StateCameraPosition::GameOver == *state.get()
    }
}
