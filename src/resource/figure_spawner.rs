use crate::{
    components::{figures::spawner::random_spawn_figure, map::TILE_SIZE},
    states::StateGame,
};
use bevy::{prelude::*, utils::HashSet};

const SIZE: usize = 3;

const FIGURE_POSITIONS: [(f32, f32); SIZE] = [
    (TILE_SIZE * 3.45, TILE_SIZE * -7.),
    (0., TILE_SIZE * -7.),
    (TILE_SIZE * -3.45, TILE_SIZE * -7.),
];

#[derive(Resource, Default)]
pub struct FigureSpawner {
    pub figures: HashSet<Entity>,
}

pub fn spawn_figures(mut commands: Commands, mut figure_spawner: ResMut<FigureSpawner>) {
    if figure_spawner.figures.is_empty() {
        for &_position in FIGURE_POSITIONS.iter() {
            for &position in FIGURE_POSITIONS.iter() {
                figure_spawner.figures.insert(random_spawn_figure(
                    &mut commands,
                    Vec2::new(position.0, position.1),
                ));
            }
        }
    }
}

pub fn despawn_figures(
    mut commands: Commands,
    mut figure_spawner: ResMut<FigureSpawner>,
    state: Res<State<StateGame>>,
    mut next_state: ResMut<NextState<StateGame>>,
) {
    if let StateGame::Placed(entity) = state.get() {
        commands.entity(*entity).despawn();
        figure_spawner.figures.remove(entity);
        spawn_figures(commands, figure_spawner);
        next_state.set(StateGame::CheckCombo);
    }
}
