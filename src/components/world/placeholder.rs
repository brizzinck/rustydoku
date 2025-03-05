use bevy::prelude::*;

use crate::constants::placeholder::*;

#[derive(Component)]
pub struct Placeholder;

impl Placeholder {
    pub(crate) fn create(position: (f32, f32), assets: &Res<AssetServer>) -> impl Bundle {
        (
            Sprite {
                image: assets.load(FIGURE_PLACEHOLDER_IMAGE_DEFAULT),
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
            Placeholder,
        )
    }
}
