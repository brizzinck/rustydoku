use super::Figure;
use bevy::prelude::*;

pub(crate) fn spawn(commands: &mut Commands, position: Vec2, assets: &Res<AssetServer>) -> Entity {
    Figure::spawn_figure(
        commands,
        position,
        &[
            Vec2::new(-0.5, -0.5),
            Vec2::new(0.5, 0.5),
            Vec2::new(-0.5, 0.5),
            Vec2::new(0.5, -0.5),
        ],
        "cube",
        assets,
    )
}
