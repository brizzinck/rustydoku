use bevy::prelude::*;

use crate::{
    components::{
        figure::Figure,
        ui::header::HeaderUI,
        world::{figure_zone::FigureZone, placeholder::Placeholder},
    },
    constants::figure::FIGURE_POSITION_Z,
    events::{figure::FigureSpawned, figure_spawner::SpawnFigure},
    resource::figure_spawner::FigureSpawner,
    states::gameplay::StateGame,
};

pub mod init;

impl FigureSpawner {
    pub(crate) fn spawn_figures(
        mut commands: Commands,
        mut figure_spawner: ResMut<FigureSpawner>,
        assets: Res<AssetServer>,
        figure_zone: Query<Entity, With<FigureZone>>,
        placeholder: Query<(Entity, &Transform), With<Placeholder>>,
        mut event_writer: EventWriter<FigureSpawned>,
        mut event_reader: EventReader<SpawnFigure>,
    ) {
        if figure_spawner.figures.is_empty() && event_reader.read().last().is_some() {
            for (entity, &transform) in placeholder.iter() {
                let entity = Figure::random_spawn(
                    &mut commands,
                    Vec2::new(transform.translation.x, transform.translation.y),
                    &assets,
                    entity,
                );

                commands.entity(entity).set_parent(figure_zone.single());

                figure_spawner.figures.insert(
                    entity,
                    Vec3::new(
                        transform.translation.x,
                        transform.translation.y,
                        FIGURE_POSITION_Z,
                    ),
                );

                event_writer.send(FigureSpawned(entity));
            }
        }
    }

    pub(crate) fn despawn_figure(
        mut commands: Commands,
        mut figure_spawner: ResMut<FigureSpawner>,
        state: Res<State<StateGame>>,
        mut next_state: ResMut<NextState<StateGame>>,
        mut event_writer: EventWriter<SpawnFigure>,
    ) {
        if let StateGame::Placed(entity) = state.get() {
            commands.entity(*entity).despawn();
            figure_spawner.figures.remove(entity);
            event_writer.send(SpawnFigure);
            next_state.set(StateGame::CheckCombo);
        }
    }

    pub(crate) fn respawn_figures(
        mut commands: Commands,
        mut figure_spawner: ResMut<FigureSpawner>,
        mut event_writer: EventWriter<SpawnFigure>,
    ) {
        for (entity, _) in figure_spawner.figures.iter() {
            commands.entity(*entity).despawn_recursive();
        }

        figure_spawner.figures.clear();
        event_writer.send(SpawnFigure);
    }

    pub(crate) fn clear_figures(mut commands: Commands, mut figure_spawner: ResMut<FigureSpawner>) {
        for (entity, _) in figure_spawner.figures.iter() {
            commands.entity(*entity).despawn_recursive();
        }
        figure_spawner.figures.clear();
    }

    pub(crate) fn hide_figures(
        mut visibility: Query<&mut Visibility, (With<FigureZone>, Without<HeaderUI>)>,
    ) {
        for mut vis in visibility.iter_mut() {
            *vis = Visibility::Hidden;
        }
    }

    pub(crate) fn show_figures(
        mut visibility: Query<&mut Visibility, (With<FigureZone>, Without<HeaderUI>)>,
    ) {
        for mut vis in visibility.iter_mut() {
            *vis = Visibility::Visible;
        }
    }
}
