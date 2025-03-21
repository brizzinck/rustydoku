use bevy::prelude::*;

#[derive(Default, States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameOverPanelState {
    Showing,
    Showed,
    Hidding,
    #[default]
    Hidden,
}

impl GameOverPanelState {
    pub fn when_showing(state: Res<State<GameOverPanelState>>) -> bool {
        GameOverPanelState::Showing == *state.get()
    }

    pub fn when_showed(state: Res<State<GameOverPanelState>>) -> bool {
        GameOverPanelState::Showed == *state.get()
    }

    pub fn when_hidding(state: Res<State<GameOverPanelState>>) -> bool {
        GameOverPanelState::Hidding == *state.get()
    }

    pub fn when_hidded(state: Res<State<GameOverPanelState>>) -> bool {
        GameOverPanelState::Hidden == *state.get()
    }
}
