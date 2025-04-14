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
    /// Spawns a new set of figures if the figure list is currently empty and a spawn event is received.
    ///
    /// For each placeholder, a new figure is generated and positioned using the placeholder's transform.
    /// The spawned figures are parented to the `FigureZoneComponent` entity and stored in the resource.
    /// A [`FigureSpawnedEvent`] is emitted for each new figure.
    ///
    /// # Parameters
    /// - `commands`: Used to spawn new entities and attach components.
    /// - `figure_spawner`: Mutable access to the resource tracking spawned figures.
    /// - `figure_zone`: Query to get the entity representing the figure zone.
    /// - `placeholder`: Query to get each placeholder entity and its position.
    /// - `event_writer`: Event writer to emit `FigureSpawnedEvent`s.
    /// - `event_reader`: Event reader to check for the presence of a `SpawnFigureEvent`.
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

    /// Despawns a figure after it has been placed on the map.
    ///
    /// Called when the [`GameState`] is `Placed`. Removes the figure from the scene and the resource.
    /// Triggers a new [`SpawnFigureEvent`] and transitions to `CheckCombo` state.
    ///
    /// # Parameters
    /// - `commands`: Used to despawn the placed figure.
    /// - `figure_spawner`: Resource storing currently active figures.
    /// - `state`: Current game state.
    /// - `next_state`: Used to update the game state.
    /// - `event_writer`: Used to emit a new `SpawnFigureEvent`.
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

    /// Despawns all existing figures and requests a respawn after game over.
    ///
    /// Called when restarting the game after a game over.
    ///
    /// # Parameters
    /// - `commands`: Used to recursively despawn each figure.
    /// - `figure_spawner`: Resource storing currently active figures.
    /// - `event_writer`: Used to emit a new `SpawnFigureEvent`.
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

    /// Clears all currently active figures without requesting a respawn.
    ///
    /// Used during cleanup phases such as returning to the main menu or full reset.
    ///
    /// # Parameters
    /// - `commands`: Used to recursively despawn figure entities.
    /// - `figure_spawner`: Resource tracking active figures.
    pub(crate) fn clear_figures(
        mut commands: Commands,
        mut figure_spawner: ResMut<FigureSpawnerResource>,
    ) {
        for (entity, _) in figure_spawner.figures.iter() {
            commands.entity(*entity).despawn_recursive();
        }
        figure_spawner.figures.clear();
    }

    /// Hides all figure-related UI and elements by setting visibility to `Hidden`.
    ///
    /// Typically called when the game is over to hide the figure zone.
    ///
    /// # Parameters
    /// - `visibility`: Query to access figure zone UI entities and hide them.
    pub(crate) fn hide_figures(
        mut visibility: Query<&mut Visibility, (With<FigureZoneComponent>, Without<HeaderUI>)>,
    ) {
        for mut vis in visibility.iter_mut() {
            *vis = Visibility::Hidden;
        }
    }

    /// Shows all figure-related UI and elements by setting visibility to `Visible`.
    ///
    /// Typically called when resuming the game after a game over or returning to gameplay view.
    ///
    /// # Parameters
    /// - `visibility`: Query to access figure zone UI entities and show them.
    pub(crate) fn show_figures(
        mut visibility: Query<&mut Visibility, (With<FigureZoneComponent>, Without<HeaderUI>)>,
    ) {
        for mut vis in visibility.iter_mut() {
            *vis = Visibility::Visible;
        }
    }
}
