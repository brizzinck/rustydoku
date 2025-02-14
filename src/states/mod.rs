use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum StateGame {
    #[default]
    GenerateWorld,
    Idle,
    Dragging(Entity),
    Placing(Entity),
    Placed(Entity),
    CheckCombo,
    CheckGameOver,
    GameOver,
    Restart,
}
