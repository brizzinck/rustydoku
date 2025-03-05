use crate::constants::world::{
    assets::BACKGROUND_IMAGE_PATH, background::BACKGROUND_DEFAULT_POSITION,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Background;

impl Background {
    pub(crate) fn create_background(assets: &Res<AssetServer>) -> impl Bundle {
        (
            Sprite {
                image: assets.load(BACKGROUND_IMAGE_PATH),
                ..default()
            },
            Transform {
                translation: BACKGROUND_DEFAULT_POSITION,
                scale: Vec3::ONE,
                ..default()
            },
            Background,
        )
    }
}
