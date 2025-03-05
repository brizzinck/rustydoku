use bevy::prelude::*;

use crate::constants::figure_zone::SPAWN_ZONE_NAME_HIERARCHY;

#[derive(Component)]
pub struct FigureZone;

impl FigureZone {
    pub(crate) fn create() -> impl Bundle {
        (
            Name::new(SPAWN_ZONE_NAME_HIERARCHY),
            Transform::from_translation(Vec3::ZERO),
            Visibility::Inherited,
            FigureZone,
        )
    }
}
