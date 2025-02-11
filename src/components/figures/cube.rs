use super::spawner::spawn_figure;
use bevy::prelude::*;

pub fn spawn(commands: &mut Commands, position: Vec2) -> Entity {
    spawn_figure(
        commands,
        position,
        &[
            Vec2::new(-0.5, -0.5),
            Vec2::new(0.5, 0.5),
            Vec2::new(-0.5, 0.5),
            Vec2::new(0.5, -0.5),
        ],
    )
}
