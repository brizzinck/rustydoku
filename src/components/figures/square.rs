use crate::constants::figure::*;
use assets::SQAURE_IMAGE_PATH;
use bevy::prelude::*;
use square::*;

#[derive(Component, Default)]
pub struct Square {
    pub(crate) parent: Option<Entity>,
}

impl Square {
    pub(crate) fn create_child(
        parent: Entity,
        position: Vec2,
        rotation: Quat,
        assets: &Res<AssetServer>,
    ) -> impl Bundle {
        (
            Sprite {
                image: assets.load(SQAURE_IMAGE_PATH),
                custom_size: Some(Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                color: SQUARE_COLOR_DEFAULT.into(),
                ..default()
            },
            Transform {
                translation: Vec3::new(
                    position.x * SQUARE_SIZE,
                    position.y * SQUARE_SIZE,
                    FIGURE_POSITION_Z,
                ),
                rotation: rotation.inverse(),
                ..Default::default()
            },
            Square {
                parent: Some(parent),
            },
        )
    }
}
