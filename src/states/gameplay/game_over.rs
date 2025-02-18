use bevy::prelude::*;

#[derive(Default, States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum StateGameOverPanel {
    Showing,
    Showed,
    Hidding,
    #[default]
    Hidded,
}

impl StateGameOverPanel {
    pub fn when_showing(state: Res<State<StateGameOverPanel>>) -> bool {
        StateGameOverPanel::Showing == *state.get()
    }

    pub fn when_showed(state: Res<State<StateGameOverPanel>>) -> bool {
        StateGameOverPanel::Showed == *state.get()
    }

    pub fn when_hidding(state: Res<State<StateGameOverPanel>>) -> bool {
        StateGameOverPanel::Hidding == *state.get()
    }

    pub fn when_hidded(state: Res<State<StateGameOverPanel>>) -> bool {
        StateGameOverPanel::Hidded == *state.get()
    }
}

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
