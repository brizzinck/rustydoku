use super::Figure;
use bevy::prelude::*;

pub(crate) fn spawn(commands: &mut Commands, position: Vec2, assets: &Res<AssetServer>) -> Entity {
    Figure::spawn_figure(
        commands,
        position,
        &[
            Vec2::new(-1.0, 1.),
            Vec2::new(0.0, 1.),
            Vec2::new(1.0, 1.),
            Vec2::new(0.0, -0.),
            Vec2::new(0.0, -1.),
        ],
        "big_t_shape",
        assets,
    )
}
