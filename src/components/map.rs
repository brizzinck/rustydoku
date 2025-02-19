use crate::{
    constants::map::{assets::TILE_IMAGE_PATH, MAP_NAME_HIERARCHY, TILE_SIZE},
    resource::map::Map,
};
use bevy::prelude::*;
#[cfg(feature = "debug-inspector")]
use bevy_inspector_egui::prelude::*;

#[derive(Component, Default)]
#[cfg_attr(feature = "debug-inspector", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "debug-inspector", reflect(Component, InspectorOptions))]
pub struct Tile {
    pub(crate) default_color: Color,
    pub(crate) square: Option<Entity>,
}

impl Tile {
    pub(crate) fn create_tile(
        sprite_color: Color,
        position: Vec3,
        assets: &Res<AssetServer>,
    ) -> impl Bundle {
        (
            Name::new(format!("Tile ({}, {})", position.x, position.y)),
            Sprite {
                color: sprite_color,
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                image: assets.load(TILE_IMAGE_PATH),
                ..default()
            },
            Transform::from_translation(position),
            GlobalTransform::default(),
            InheritedVisibility::default(),
            Tile {
                default_color: sprite_color,
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
