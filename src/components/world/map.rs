use crate::{
    constants::map::{MAP_NAME_HIERARCHY, TILE_COLOR, TILE_SIZE},
    resource::map::MapComponent,
};
use bevy::prelude::*;

#[cfg(feature = "debug-inspector")]
use bevy_inspector_egui::prelude::*;

/// Component representing a tile in the game grid.
/// Stores the tile's default image and an optional attached square.
#[derive(Component, Default)]
#[cfg_attr(feature = "debug-inspector", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "debug-inspector", reflect(Component, InspectorOptions))]
pub struct TileComponent {
    pub(crate) default_image: Handle<Image>,
    pub(crate) square: Option<Entity>,
}

impl TileComponent {
    /// Creates a tile at a given world `position` with a specific `image`.
    ///
    /// # Parameters
    /// - `image`: The texture to apply to the tile.
    /// - `position`: World position to place the tile.
    ///
    /// # Returns
    /// A bundle containing `Sprite`, `Transform`, `TileComponent`, and metadata.
    pub(crate) fn create_tile(image: Handle<Image>, position: Vec3) -> impl Bundle {
        (
            Name::new(format!("Tile ({}, {})", position.x, position.y)),
            Sprite {
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                image: image.clone(),
                color: TILE_COLOR,
                ..default()
            },
            Transform::from_translation(position),
            GlobalTransform::default(),
            InheritedVisibility::default(),
            TileComponent {
                default_image: image,
                square: None,
            },
        )
    }
}

impl MapComponent {
    /// Creates the root map entity that holds all tiles as children.
    ///
    /// # Returns
    /// A bundle with `Name`, `Transform`, and visibility for the map root.
    pub(crate) fn create_map() -> impl Bundle {
        (
            Name::new(MAP_NAME_HIERARCHY),
            Transform::from_translation(Vec3::ZERO),
            InheritedVisibility::default(),
        )
    }
}
