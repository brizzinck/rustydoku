use crate::{
    components::world::{figure_zone::FigureZoneComponent, placeholder::PlaceholderComponent},
    constants::placeholder::PLACEHOLDER_POSITIONS,
    events::figure_spawner::SpawnFigureEvent,
    resource::figure_spawner::FigureSpawnerResource,
    states::figure::placeholder::PlaceholderAnimationState,
};
use bevy::prelude::*;

impl FigureSpawnerResource {
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
