use crate::{
    components::{figure_zone::FigureZone, placeholder::Placeholder},
    constants::placeholder::PLACEHOLDER_POSITION,
    resource::figure_spawner::FigureSpawner,
    states::figure::placeholder::StatePlaceholderAnimation,
};
use bevy::prelude::*;

impl FigureSpawner {
    pub(crate) fn spawn_zone_figures(
        mut commands: Commands,
        assets: Res<AssetServer>,
        mut next_state: ResMut<NextState<StatePlaceholderAnimation>>,
    ) {
        let parent = commands.spawn(FigureZone::create()).id();

        for &position in PLACEHOLDER_POSITION.iter() {
            commands
                .spawn(Placeholder::create(position, &assets))
                .set_parent(parent);
        }

        next_state.set(StatePlaceholderAnimation::BouncingInit);
    }
}
