use crate::{
    components::map::{restart_tiles, Tile},
    resource::{
        figure_spawner::{restart_figures, FigureSpawner},
        score::{restart_score, Score},
    },
    states::StateGame,
};
use bevy::prelude::*;

pub fn restart(
    mut commands: Commands,
    mut next_state: ResMut<NextState<StateGame>>,
    figure_spawner: ResMut<FigureSpawner>,
    score: ResMut<Score>,
    tiles: Query<&mut Tile>,
) {
    restart_figures(&mut commands, figure_spawner);
    restart_score(score);
    restart_tiles(&mut commands, tiles);

    next_state.set(StateGame::Idle);
}
