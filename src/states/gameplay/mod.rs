use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum StateGame {
    GenerateWorld,
    Idle,
    Dragging(Entity),
    Placing(Entity),
    Placed(Entity),
    CheckCombo,
    CheckGameOver,
    #[default]
    GameOver,
    Restart,
}

impl StateGame {
    pub fn when_idle(state: Res<State<StateGame>>) -> bool {
        StateGame::Idle == *state.get()
    }

    pub fn when_draggin(state: Res<State<StateGame>>) -> bool {
        matches!(state.get(), StateGame::Dragging(_))
    }

    pub fn when_placing(state: Res<State<StateGame>>) -> bool {
        matches!(state.get(), StateGame::Placing(_))
    }

    pub fn when_placed(state: Res<State<StateGame>>) -> bool {
        matches!(state.get(), StateGame::Placed(_))
    }
}

pub fn reset_state(mut state: ResMut<NextState<StateGame>>) {
    state.set(StateGame::Idle);
}
