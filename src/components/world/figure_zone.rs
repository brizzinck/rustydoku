use bevy::prelude::*;

use crate::constants::figure_zone::SPAWN_ZONE_NAME_HIERARCHY;

/// Component for identifying the parent container that holds all figure entities.
#[derive(Component)]
pub struct FigureZoneComponent;

impl FigureZoneComponent {
    /// Creates the figure zone container with default transform and visibility.
    ///
    /// # Returns
    /// A bundle containing the `FigureZoneComponent`, `Name`, `Transform`, and `Visibility`.
    pub(crate) fn create() -> impl Bundle {
        (
            Name::new(SPAWN_ZONE_NAME_HIERARCHY),
            Transform::from_translation(Vec3::ZERO),
            Visibility::Inherited,
            FigureZoneComponent,
        )
    }
}
