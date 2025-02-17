use crate::{
    logic::world::restart::restart,
    resource::figure_spawner::{clear_figures, hidden_figures},
    states::gameplay::StateGame,
};
use bevy::prelude::*;

pub struct RustydokuGameplay;

impl Plugin for RustydokuGameplay {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(StateGame::Restart), restart);
        app.add_systems(
            OnEnter(StateGame::GameOver),
            (clear_figures, hidden_figures).chain(),
        );
    }
}
