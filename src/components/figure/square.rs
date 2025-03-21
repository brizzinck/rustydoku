use crate::{
    constants::{figure::*, square::*},
    resource::figure_spawner::FigureSpawnerResource,
};
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct SquareComponent {
    pub(crate) parent: Option<Entity>,
}

impl SquareComponent {
    pub(crate) fn create_child(
        parent: Entity,
        position: Vec2,
        rotation: Quat,
        resource: &FigureSpawnerResource,
    ) -> impl Bundle {
        (
            Sprite {
                image: resource.get_square_image(),
                custom_size: Some(Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                color: SQUARE_COLOR_DEFAULT.into(),
                ..default()
            },
            Transform {
                translation: Vec3::new(
                    position.x * SQUARE_SIZE,
                    position.y * SQUARE_SIZE,
                    FIGURE_Z_POSITION,
                ),
                rotation: rotation.inverse(),
                ..Default::default()
            },
            SquareComponent {
                parent: Some(parent),
            },
        )
    }
}
