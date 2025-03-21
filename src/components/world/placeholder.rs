use bevy::prelude::*;

use crate::{constants::placeholder::*, resource::figure_spawner::FigureSpawnerResource};

#[derive(Component)]
pub struct PlaceholderComponent;

impl PlaceholderComponent {
    pub(crate) fn create(position: (f32, f32), resource: &FigureSpawnerResource) -> impl Bundle {
        (
            Sprite {
                image: resource.get_placeholder_image(),
                custom_size: Some(PLACEHOLDER_SIZE),
                color: PLACEHOLDER_COLOR.into(),
                ..default()
            },
            Transform {
                translation: Vec3::new(position.0, position.1, PLACEHOLDER_Z_POSITION),
                scale: PLACEHOLDER_SCALE_INITIAL,
                ..default()
            },
            Name::new(PLACEHOLDER_NAME_HIERARCHY),
            PlaceholderComponent,
        )
    }
}
