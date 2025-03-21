use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    InitResources,
    GenerateWorld,
    Idle,
    Dragging(Entity),
    Placing(Entity),
    Placed(Entity),
    CheckCombo,
    CheckGameOver,
    GameOver,
    DefaultRestart,
    GameOverRestart,
}

impl GameState {
    pub fn when_idle(state: Res<State<GameState>>) -> bool {
        GameState::Idle == *state.get()
    }

    pub fn when_draggin(state: Res<State<GameState>>) -> bool {
        matches!(state.get(), GameState::Dragging(_))
    }

    pub fn when_placing(state: Res<State<GameState>>) -> bool {
        matches!(state.get(), GameState::Placing(_))
    }

    pub fn when_placed(state: Res<State<GameState>>) -> bool {
        matches!(state.get(), GameState::Placed(_))
    }
}
