use bevy::prelude::*;

use crate::{constants::placeholder::*, resource::figure_spawner::FigureSpawnerResource};

/// Marker component for a figure placeholder in the game zone.
#[derive(Component)]
pub struct PlaceholderComponent;

impl PlaceholderComponent {
    /// Creates a new placeholder sprite bundle at a given `(x, y)` position
    /// using the provided placeholder image from the `FigureSpawnerResource`.
    ///
    /// # Parameters
    /// - `position`: The `(x, y)` coordinate to place the placeholder.
    /// - `resource`: Reference to the figure spawner resource to access assets.
    ///
    /// # Returns
    /// A bundle that includes a `Sprite`, `Transform`, `Name`, and the `PlaceholderComponent`.
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
