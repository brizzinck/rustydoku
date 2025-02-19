use crate::logic::figure::spawner::spawn_figure;
use bevy::prelude::*;

pub(crate) fn spawn(commands: &mut Commands, position: Vec2, assets: &Res<AssetServer>) -> Entity {
    spawn_figure(
        commands,
        position,
        &[
            Vec2::new(-1.0, 0.5),
            Vec2::new(0.0, 0.5),
            Vec2::new(1.0, 0.5),
            Vec2::new(0.0, -0.5),
        ],
        "t_shape",
        assets,
    )
}
