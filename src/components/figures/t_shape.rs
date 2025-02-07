use super::{
    spawner::spawn_empty_figure,
    square::{self},
};
use bevy::prelude::*;

pub fn spawn(commands: &mut Commands, position: Vec2) {
    let parent = spawn_empty_figure(commands, position);

    let positions = [
        Vec2::new(-1.0, 0.0), // Left block
        Vec2::new(0.0, 0.0),  // Center block
        Vec2::new(1.0, 0.0),  // Right block
        Vec2::new(0.0, -1.0),
    ];

    for &pos in &positions {
        square::spawn_child(commands, parent, pos);
    }
}
