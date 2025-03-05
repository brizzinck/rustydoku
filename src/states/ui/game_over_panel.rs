use bevy::prelude::*;

#[derive(Default, States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum StateGameOverPanel {
    Showing,
    Showed,
    Hidding,
    #[default]
    Hidden,
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
        StateGameOverPanel::Hidden == *state.get()
    }
}
