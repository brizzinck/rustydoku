use super::{
    spawner::spawn_empty_figure,
    square::{self},
};
use bevy::prelude::*;

pub fn spawn(commands: &mut Commands, position: Vec2) {
    let squares_position = [
        Vec2::new(-1.0, 0.0),
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(0.0, -1.0),
    ];

    let parent = spawn_empty_figure(commands, position, &squares_position);

    for &offset in &squares_position {
        square::spawn_child(commands, parent, offset);
    }
}
