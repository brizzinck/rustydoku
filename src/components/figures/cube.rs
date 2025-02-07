use super::{spawner::spawn_empty_figure, square::spawn_child};
use bevy::prelude::*;

pub fn spawn(commands: &mut Commands, position: Vec2) {
    let squares_position = vec![
        Vec2::new(-0.5, -0.5),
        Vec2::new(0.5, 0.5),
        Vec2::new(-0.5, 0.5),
        Vec2::new(0.5, -0.5),
    ];

    let parent = spawn_empty_figure(commands, position, &squares_position);

    for &offset in &squares_position {
        spawn_child(commands, parent, offset);
    }
}
