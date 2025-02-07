use super::{spawner::spawn_empty_figure, square::spawn_child};
use bevy::prelude::*;

pub fn spawn(commands: &mut Commands, position: Vec2) {
    let parent = spawn_empty_figure(commands, position);
    for x in -1..1 {
        for y in -1..1 {
            spawn_child(commands, parent, Vec2::new(x as f32 + 0.5, y as f32 + 0.5));
        }
    }
}
