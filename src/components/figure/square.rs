use crate::{
    constants::{figure::*, square::*},
    resource::figure_spawner::FigureSpawnerResource,
};
use bevy::prelude::*;

/// Component representing an individual square belonging to a figure.
#[derive(Component, Default)]
pub struct SquareComponent {
    /// Optional reference to the parent entity (usually the figure).
    pub(crate) parent: Option<Entity>,
}

impl SquareComponent {
    /// Creates a child square component as part of a figure.
    ///
    /// # Arguments
    /// - `parent`: The parent figure entity to which this square belongs.
    /// - `position`: The relative 2D position of the square within the figure.
    /// - `rotation`: The rotation of the square, typically the inverse of the figure's rotation.
    /// - `resource`: Reference to the figure spawner resource, used to access the square image.
    ///
    /// # Returns
    /// A `Bundle` representing the square with a sprite, transform, and component marker.
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
