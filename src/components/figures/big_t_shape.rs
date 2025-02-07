use super::{spawner::spawn_empty_figure, square};
use bevy::prelude::*;

pub fn spawn(commands: &mut Commands, position: Vec2) {
    let parent = spawn_empty_figure(commands, position);

    let positions = [
        Vec2::new(-1.0, 0.0),
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(0.0, -1.0),
        Vec2::new(0.0, -2.0),
    ];
    for &pos in &positions {
        square::spawn_child(commands, parent, pos);
    }
}
