use crate::{
    components::{
        map::{restart_tiles, Tile},
        ui::{
            game_over::{despawn_game_over_panel, GameOverPanel},
            header::{reset_header, HeaderUI},
        },
    },
    resource::{
        figure_spawner::{restart_figures, show_figures, FigureSpawner, FigureZone},
        score::{restart_score, Score},
    },
    states::gameplay::StateGame,
};
use bevy::prelude::*;

#[allow(clippy::too_many_arguments)] //TODO: Refactor this function, extracting
pub fn restart(
    mut commands: Commands,
    mut next_state: ResMut<NextState<StateGame>>,
    assets: Res<AssetServer>,
    figure_spawner: ResMut<FigureSpawner>,
    score: ResMut<Score>,
    tiles: Query<&mut Tile>,
    game_over_panel: Query<Entity, With<GameOverPanel>>,
    mut header: Query<&mut Visibility, With<HeaderUI>>,
    mut figures_zone: Query<&mut Visibility, (With<FigureZone>, Without<HeaderUI>)>,
) {
    info!("Restarting game");

    restart_figures(&mut commands, figure_spawner, assets);
    restart_score(score);
    restart_tiles(&mut commands, tiles);
    despawn_game_over_panel(&mut commands, game_over_panel);
    reset_header(&mut header);
    show_figures(&mut figures_zone);

    next_state.set(StateGame::Idle);
}
