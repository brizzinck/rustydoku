use super::Figure;
use crate::{
    constants::figure::*,
    logic::figure::{spawner::spawn_empty_figure, square::spawn_as_child},
    states::figure::StateFigureAnimation,
};
use assets::SQAURE_IMAGE_PATH;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Square {
    pub(crate) parent: Option<Entity>,
}

pub fn spawn(commands: &mut Commands, position: Vec2, assets: &Res<AssetServer>) -> Entity {
    let (parent, rotation) = spawn_empty_figure(commands, position, &[Vec2::new(0., 0.)]);
    let child = spawn_as_child(commands, parent, Vec2::new(0., 0.), rotation, assets);

    commands.entity(parent).insert(Figure {
        squares_entity: vec![child],
        squares_position: vec![Vec2::new(0., 0.)],
        state_animation: StateFigureAnimation::default(),
    });
    commands.entity(parent).insert(Name::new("square"));

    parent
}

pub fn create_child(
    parent: Entity,
    position: Vec2,
    rotation: Quat,
    assets: &Res<AssetServer>,
) -> impl Bundle {
    (
        Sprite {
            image: assets.load(SQAURE_IMAGE_PATH),
            custom_size: Some(Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
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
