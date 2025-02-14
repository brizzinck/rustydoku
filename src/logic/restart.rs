use crate::{
    components::{
        map::{restart_tiles, Tile},
        ui::game_over::{despawn_game_over_panel, GameOverPanel},
    },
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
    assets: Res<AssetServer>,
    figure_spawner: ResMut<FigureSpawner>,
    score: ResMut<Score>,
    tiles: Query<&mut Tile>,
    game_over_panel: Query<Entity, With<GameOverPanel>>,
) {
    info!("Restarting game");

    restart_figures(&mut commands, figure_spawner, assets);
    restart_score(score);
    restart_tiles(&mut commands, tiles);
    despawn_game_over_panel(&mut commands, game_over_panel);

    next_state.set(StateGame::Idle);
}
