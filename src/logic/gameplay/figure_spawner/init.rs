use crate::{
    components::figures::{FigureZone, Placeholder},
    constants::figure::{
        assets::FIGURE_PLACEHOLDER_DEFAULT_IMAGE_PATH, placeholder::*,
        spawn_zone::SPAWN_ZONE_NAME_HIERARCHY,
    },
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
        let parent = commands
            .spawn((
                Name::new(SPAWN_ZONE_NAME_HIERARCHY),
                Transform::from_translation(Vec3::ZERO),
                Visibility::Inherited,
                FigureZone,
            ))
            .id();

        for &position in PLACEHOLDER_POSITION.iter() {
            commands
                .spawn((
                    Sprite {
                        image: assets.load(FIGURE_PLACEHOLDER_DEFAULT_IMAGE_PATH),
                        custom_size: Some(PLACEHOLDER_SIZE),
                        color: PLACEHOLDER_COLOR.into(),
                        ..default()
                    },
                    Transform {
                        translation: Vec3::new(position.0, position.1, PLACEHOLDER_POSITION_Z),
                        scale: PLACEHOLDER_SCALE_INITIAL,
                        ..default()
                    },
                    Name::new(PLACEHOLDER_NAME_HIERARCHY),
                    Placeholder,
                ))
                .set_parent(parent);
        }

        next_state.set(StatePlaceholderAnimation::BouncingInit);
    }
}
