use bevy::prelude::*;

use crate::{
    components::{figures::FigureZone, ui::header::HeaderUI},
    constants::figure::{placeholder::PLACEHOLDER_POSITION, FIGURE_POSITION_Z},
    logic::figure::spawner::random_spawn_figure,
    resource::figure_spawner::FigureSpawner,
    states::gameplay::StateGame,
};

pub mod animation;
pub mod init;

pub fn spawn_figures(
    mut commands: Commands,
    mut figure_spawner: ResMut<FigureSpawner>,
    assets: Res<AssetServer>,
    figure_zone: Query<Entity, With<FigureZone>>,
) {
    if figure_spawner.figures.is_empty() {
        for &position in PLACEHOLDER_POSITION.iter() {
            let entity =
                random_spawn_figure(&mut commands, Vec2::new(position.0, position.1), &assets);
            commands.entity(entity).set_parent(figure_zone.single());
            figure_spawner
                .figures
                .insert(entity, Vec3::new(position.0, position.1, FIGURE_POSITION_Z));
        }
    }
}

pub fn despawn_figure(
    mut commands: Commands,
    mut figure_spawner: ResMut<FigureSpawner>,
    state: Res<State<StateGame>>,
    mut next_state: ResMut<NextState<StateGame>>,
    assets: Res<AssetServer>,
    figure_zone: Query<Entity, With<FigureZone>>,
) {
    if let StateGame::Placed(entity) = state.get() {
        commands.entity(*entity).despawn();
        figure_spawner.figures.remove(entity);
        spawn_figures(commands, figure_spawner, assets, figure_zone);
        next_state.set(StateGame::CheckCombo);
    }
}

pub fn respawn_figures(
    mut commands: Commands,
    mut figure_spawner: ResMut<FigureSpawner>,
    assets: Res<AssetServer>,
    figure_zone: Query<Entity, With<FigureZone>>,
) {
    for (entity, _) in figure_spawner.figures.iter() {
        commands.entity(*entity).despawn_recursive();
    }

    figure_spawner.figures.clear();
    spawn_figures(commands.reborrow(), figure_spawner, assets, figure_zone);
}

pub fn clear_figures(mut commands: Commands, mut figure_spawner: ResMut<FigureSpawner>) {
    for (entity, _) in figure_spawner.figures.iter() {
        commands.entity(*entity).despawn_recursive();
    }
    figure_spawner.figures.clear();
}

pub fn hide_figures(mut visibility: Query<&mut Visibility, (With<FigureZone>, Without<HeaderUI>)>) {
    for mut vis in visibility.iter_mut() {
        *vis = Visibility::Hidden;
    }
}

pub fn show_figures(mut visibility: Query<&mut Visibility, (With<FigureZone>, Without<HeaderUI>)>) {
    for mut vis in visibility.iter_mut() {
        *vis = Visibility::Visible;
    }
}
