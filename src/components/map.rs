use crate::{
    constants::map::{MAP_NAME_HIERARCHY, TILE_SIZE},
    resource::map::Map,
};
use bevy::prelude::*;
#[cfg(feature = "debug-inspector")]
use bevy_inspector_egui::prelude::*;

#[derive(Component, Default)]
#[cfg_attr(feature = "debug-inspector", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "debug-inspector", reflect(Component, InspectorOptions))]
pub struct Tile {
    pub(crate) default_image: Handle<Image>,
    pub(crate) square: Option<Entity>,
}

impl Tile {
    pub(crate) fn create_tile(image: Handle<Image>, position: Vec3) -> impl Bundle {
        (
            Name::new(format!("Tile ({}, {})", position.x, position.y)),
            Sprite {
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                image: image.clone(),
                ..default()
            },
            Transform::from_translation(position),
            GlobalTransform::default(),
            InheritedVisibility::default(),
            Tile {
                default_image: image,
                square: None,
            },
        )
    }
}

impl Map {
    pub(crate) fn create_map() -> impl Bundle {
        (
            Name::new(MAP_NAME_HIERARCHY),
            Transform::from_translation(Vec3::ZERO),
            InheritedVisibility::default(),
        )
    }
}
