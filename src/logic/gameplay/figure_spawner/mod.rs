use bevy::prelude::*;

use crate::{
    components::{
        figure::FigureComponent,
        ui::header::HeaderUI,
        world::{figure_zone::FigureZoneComponent, placeholder::PlaceholderComponent},
    },
    constants::figure::FIGURE_Z_POSITION,
    events::{figure::FigureSpawnedEvent, figure_spawner::SpawnFigureEvent},
    resource::figure_spawner::FigureSpawnerResource,
    states::gameplay::GameState,
};

pub mod init;

impl FigureSpawnerResource {
    pub(crate) fn spawn_figures(
        mut commands: Commands,
        mut figure_spawner: ResMut<FigureSpawnerResource>,
        figure_zone: Query<Entity, With<FigureZoneComponent>>,
        placeholder: Query<(Entity, &Transform), With<PlaceholderComponent>>,
        mut event_writer: EventWriter<FigureSpawnedEvent>,
        mut event_reader: EventReader<SpawnFigureEvent>,
    ) {
        if figure_spawner.figures.is_empty() && event_reader.read().last().is_some() {
            for (entity, &transform) in placeholder.iter() {
                let entity = FigureComponent::random_spawn(
                    &mut commands,
                    Vec2::new(transform.translation.x, transform.translation.y),
                    &figure_spawner,
                    entity,
                );

                commands.entity(entity).set_parent(figure_zone.single());

                figure_spawner.figures.insert(
                    entity,
                    Vec3::new(
                        transform.translation.x,
                        transform.translation.y,
                        FIGURE_Z_POSITION,
                    ),
                );

                event_writer.send(FigureSpawnedEvent(entity));
            }
        }
    }

    pub(crate) fn despawn_figure(
        mut commands: Commands,
        mut figure_spawner: ResMut<FigureSpawnerResource>,
        state: Res<State<GameState>>,
        mut next_state: ResMut<NextState<GameState>>,
        mut event_writer: EventWriter<SpawnFigureEvent>,
    ) {
        if let GameState::Placed(entity) = state.get() {
            commands.entity(*entity).despawn();
            figure_spawner.figures.remove(entity);
            event_writer.send(SpawnFigureEvent);
            next_state.set(GameState::CheckCombo);
        }
    }

    pub(crate) fn respawn_figures(
        mut commands: Commands,
        mut figure_spawner: ResMut<FigureSpawnerResource>,
        mut event_writer: EventWriter<SpawnFigureEvent>,
    ) {
        for (entity, _) in figure_spawner.figures.iter() {
            commands.entity(*entity).despawn_recursive();
        }

        figure_spawner.figures.clear();
        event_writer.send(SpawnFigureEvent);
    }

    pub(crate) fn clear_figures(
        mut commands: Commands,
        mut figure_spawner: ResMut<FigureSpawnerResource>,
    ) {
        for (entity, _) in figure_spawner.figures.iter() {
            commands.entity(*entity).despawn_recursive();
        }
        figure_spawner.figures.clear();
    }

    pub(crate) fn hide_figures(
        mut visibility: Query<&mut Visibility, (With<FigureZoneComponent>, Without<HeaderUI>)>,
    ) {
        for mut vis in visibility.iter_mut() {
            *vis = Visibility::Hidden;
        }
    }

    pub(crate) fn show_figures(
        mut visibility: Query<&mut Visibility, (With<FigureZoneComponent>, Without<HeaderUI>)>,
    ) {
        for mut vis in visibility.iter_mut() {
            *vis = Visibility::Visible;
        }
    }
}
