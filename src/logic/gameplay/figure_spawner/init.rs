use bevy::prelude::*;

use crate::{
    components::figures::FigureZone,
    constants::figure::{
        assets::FIGURE_PLACEHOLDER_IMAGE_PATH,
        placeholder::{
            PLACEHOLDER_COLOR, PLACEHOLDER_NAME_HIERARCHY, PLACEHOLDER_POSITION,
            PLACEHOLDER_POSITION_Z, PLACEHOLDER_SIZE,
        },
        spawn_zone::SPAWN_ZONE_NAME_HIERARCHY,
    },
};

pub fn spawn_zone_figures(mut commands: Commands, assets: Res<AssetServer>) {
    let parent = commands
        .spawn((
            Name::new(SPAWN_ZONE_NAME_HIERARCHY),
            Transform::from_translation(Vec3::ZERO),
            Visibility::Inherited,
            FigureZone,
        ))
        .id();

    for &position in PLACEHOLDER_POSITION.iter() {
        commands.entity(parent).with_children(|parent| {
            parent.spawn((
                Sprite {
                    image: assets.load(FIGURE_PLACEHOLDER_IMAGE_PATH),
                    custom_size: Some(Vec2::new(PLACEHOLDER_SIZE, PLACEHOLDER_SIZE)),
                    color: PLACEHOLDER_COLOR.into(),
                    ..default()
                },
                Transform::from_translation(Vec3::new(
                    position.0,
                    position.1,
                    PLACEHOLDER_POSITION_Z,
                )),
                Name::new(PLACEHOLDER_NAME_HIERARCHY),
            ));
        });
    }
}
