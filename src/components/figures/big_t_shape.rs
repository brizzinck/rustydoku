use super::spawner::spawn_figure;
use bevy::prelude::*;

pub fn spawn(commands: &mut Commands, position: Vec2) -> Entity {
    spawn_figure(
        commands,
        position,
        &[
            Vec2::new(-1.0, 0.0),
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(0.0, -1.0),
            Vec2::new(0.0, -2.0),
        ],
    )
}
