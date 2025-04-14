use crate::{
    components::world::{figure_zone::FigureZoneComponent, placeholder::PlaceholderComponent},
    constants::placeholder::PLACEHOLDER_POSITIONS,
    events::figure_spawner::SpawnFigureEvent,
    resource::figure_spawner::FigureSpawnerResource,
    states::figure::placeholder::PlaceholderAnimationState,
};
use bevy::prelude::*;

impl FigureSpawnerResource {
    /// Spawns the initial figure zone and its placeholders at the start of the game.
    ///
    /// This method:
    /// - Creates a parent entity for the figure zone using [`FigureZoneComponent`].
    /// - Spawns a set of placeholders at predefined positions ([`PLACEHOLDER_POSITIONS`]).
    /// - Sets each placeholder's parent to the figure zone entity.
    /// - Updates the placeholder animation state to [`PlaceholderAnimationState::BouncingInit`].
    /// - Emits a [`SpawnFigureEvent`] to trigger the initial figure spawn.
    ///
    /// This is typically called once during the game setup phase.
    ///
    /// # Parameters
    /// - `commands`: Used to spawn the figure zone and placeholder entities.
    /// - `resource`: Reference to [`FigureSpawnerResource`] to access placeholder images.
    /// - `next_state`: Used to update the animation state for placeholder components.
    /// - `event_writer`: Used to emit an event to initiate figure spawning.
    pub(crate) fn spawn_zone_figures(
        mut commands: Commands,
        resource: Res<FigureSpawnerResource>,
        mut next_state: ResMut<NextState<PlaceholderAnimationState>>,
        mut event_writer: EventWriter<SpawnFigureEvent>,
    ) {
        trace!("Spawning zone figures");
        let parent = commands.spawn(FigureZoneComponent::create()).id();

        for &position in PLACEHOLDER_POSITIONS.iter() {
            commands
                .spawn(PlaceholderComponent::create(position, &resource))
                .set_parent(parent);
        }

        next_state.set(PlaceholderAnimationState::BouncingInit);
        trace!("Zone figures spawned");

        event_writer.send(SpawnFigureEvent);
        trace!("SpawnFigure event sent");
    }
}
